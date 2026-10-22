use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    env,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::{interval, Duration};
use tokio_postgres::{Config as PgConfig, NoTls};

type HmacSha256 = Hmac<Sha256>;
const SCENARIO_ID: &str = "tenant-isolation-v1";

#[derive(Clone)]
struct CanaryConfig {
    proxy: SocketAddr,
    database: String,
    schema: String,
    table: String,
    tenant_column: String,
    tenants: Vec<String>,
    tenant_signing_key: Vec<u8>,
    node_id: String,
    node_signing_key: Vec<u8>,
    api_token: String,
    register_url: String,
    report_url: String,
    interval_seconds: u64,
}

pub(crate) fn control_plane_url_allowed(url: &str, allow_insecure_http: bool) -> bool {
    url.starts_with("https://")
        || url.starts_with("http://127.0.0.1:")
        || url.starts_with("http://localhost:")
        || (allow_insecure_http && url.starts_with("http://"))
}

impl CanaryConfig {
    fn from_env(listener: SocketAddr) -> Result<Option<Self>, String> {
        if !matches!(
            env::var("PROXIMA_TENANT_CANARY_ENABLED").as_deref(),
            Ok("true" | "1" | "yes")
        ) {
            return Ok(None);
        }
        let proxy = listener;
        if !proxy.ip().is_loopback() || proxy.port() == 0 {
            return Err(
                "PROXIMA_TENANT_CANARY_LOCAL_LISTEN_ADDR must use a fixed loopback address and nonzero port".into(),
            );
        }
        let required = |name: &str| -> Result<String, String> {
            env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("missing required environment variable {name}"))
        };
        let node_id = required("PROXIMA_NODE_ID")?;
        let node_id = uuid::Uuid::parse_str(&node_id)
            .map_err(|_| {
                "PROXIMA_NODE_ID must be the enrolled node UUID for tenant canaries".to_string()
            })?
            .to_string();
        let node_signing_key = hex::decode(required("PROXIMA_NODE_SIGNING_KEY_HEX")?.trim())
            .map_err(|_| "PROXIMA_NODE_SIGNING_KEY_HEX must be valid hex")?;
        if node_signing_key.len() != 32 {
            return Err("PROXIMA_NODE_SIGNING_KEY_HEX must decode to 32 bytes".into());
        }
        let tenant_signing_key = required("PROXIMA_TENANT_SIGNING_KEY")?.into_bytes();
        if tenant_signing_key.len() < 32 {
            return Err("PROXIMA_TENANT_SIGNING_KEY must contain at least 32 bytes".into());
        }
        let api_token = env::var("PROXIMA_SECURITY_EVENT_INGEST_TOKEN")
            .or_else(|_| env::var("PROXIMA_ATTESTATION_INGEST_TOKEN"))
            .map_err(|_| {
                "missing PROXIMA_SECURITY_EVENT_INGEST_TOKEN or PROXIMA_ATTESTATION_INGEST_TOKEN"
                    .to_string()
            })?;
        if !api_token.starts_with("aga_") {
            return Err("security event ingest token must be an Agata API key (aga_...)".into());
        }
        let allow_insecure_http = matches!(
            env::var("PROXIMA_TENANT_CANARY_ALLOW_INSECURE_HTTP")
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase()
                .as_str(),
            "true" | "1" | "yes"
        );
        let event_url = required("PROXIMA_SECURITY_EVENT_INGEST_URL")?;
        if !control_plane_url_allowed(&event_url, allow_insecure_http) {
            return Err("security event ingest URL must use HTTPS; plain HTTP is allowed only for loopback development or with the explicit PROXIMA_TENANT_CANARY_ALLOW_INSECURE_HTTP opt-in".into());
        }
        let base = event_url
            .strip_suffix("/api/v1/security/events")
            .unwrap_or_else(|| event_url.trim_end_matches('/'));
        let register_url = env::var("PROXIMA_SECURITY_PROBE_REGISTER_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("{base}/api/v1/security/probes/register"));
        let report_url = env::var("PROXIMA_SECURITY_PROBE_REPORT_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("{base}/api/v1/security/probes/report"));
        for url in [&register_url, &report_url] {
            if !control_plane_url_allowed(url, allow_insecure_http) {
                return Err("security probe URLs must use HTTPS; plain HTTP is allowed only for loopback development or with the explicit PROXIMA_TENANT_CANARY_ALLOW_INSECURE_HTTP opt-in".into());
            }
        }
        let tenants = required("PROXIMA_TENANT_CANARY_TENANTS")?
            .split(',')
            .map(str::trim)
            .map(str::to_owned)
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        if tenants.len() < 2
            || tenants.len() > 10
            || tenants
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                != tenants.len()
            || tenants
                .iter()
                .map(|tenant| password_env_name(tenant))
                .collect::<std::collections::HashSet<_>>()
                .len()
                != tenants.len()
        {
            return Err("PROXIMA_TENANT_CANARY_TENANTS must contain 2–10 unique tenant IDs".into());
        }
        if tenants.iter().any(|tenant| {
            tenant.len() > 128
                || tenant.is_empty()
                || !tenant
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        }) {
            return Err(
                "tenant IDs may contain only ASCII letters, digits, hyphens and underscores".into(),
            );
        }
        for tenant in &tenants {
            let key = password_env_name(tenant);
            if env::var(&key)
                .ok()
                .filter(|value| !value.is_empty())
                .is_none()
            {
                return Err(format!(
                    "missing password for canary tenant {tenant}: set {key}"
                ));
            }
        }
        let schema =
            env::var("PROXIMA_TENANT_CANARY_SCHEMA").unwrap_or_else(|_| "proxima_security".into());
        let table =
            env::var("PROXIMA_TENANT_CANARY_TABLE").unwrap_or_else(|_| "canary_records".into());
        let tenant_column =
            env::var("PROXIMA_TENANT_CANARY_TENANT_COLUMN").unwrap_or_else(|_| "tenant_id".into());
        for identifier in [&schema, &table, &tenant_column] {
            if !valid_identifier(identifier) {
                return Err(
                    "canary schema, table and column must be simple SQL identifiers".into(),
                );
            }
        }
        let interval_seconds = env::var("PROXIMA_TENANT_CANARY_INTERVAL_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(300);
        if !(60..=3600).contains(&interval_seconds) {
            return Err(
                "PROXIMA_TENANT_CANARY_INTERVAL_SECONDS must be between 60 and 3600".into(),
            );
        }
        Ok(Some(Self {
            proxy,
            database: env::var("PROXIMA_TENANT_CANARY_DATABASE")
                .unwrap_or_else(|_| "proxima_dev".into()),
            schema,
            table,
            tenant_column,
            tenants,
            tenant_signing_key,
            node_id,
            node_signing_key,
            api_token,
            register_url,
            report_url,
            interval_seconds,
        }))
    }
}

fn valid_identifier(identifier: &str) -> bool {
    !identifier.is_empty()
        && identifier
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && identifier
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn user_env_name(tenant: &str) -> String {
    format!(
        "PROXIMA_TENANT_CANARY_USER_{}",
        tenant
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            })
            .collect::<String>()
    )
}

fn default_user_name(tenant: &str) -> String {
    format!(
        "proxima_tenant_{}",
        tenant
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect::<String>()
    )
}

fn password_env_name(tenant: &str) -> String {
    format!(
        "PROXIMA_TENANT_CANARY_PASSWORD_{}",
        tenant
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            })
            .collect::<String>()
    )
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default()
}

fn hmac_hex(key: &[u8], message: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key size");
    mac.update(message.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn tenant_token(config: &CanaryConfig, tenant: &str) -> String {
    let expires = now_unix() + 300;
    let payload = format!("v1.{tenant}.{expires}");
    format!(
        "{payload}.{}",
        hmac_hex(&config.tenant_signing_key, &payload)
    )
}

async fn register(config: &CanaryConfig, client: &reqwest::Client) -> Result<(), String> {
    let observed_at = now_unix();
    let message = format!(
        "register|{}|{}|{}|{}",
        config.node_id, SCENARIO_ID, config.interval_seconds, observed_at
    );
    let body = json!({
        "node_id": config.node_id, "scenario_id": SCENARIO_ID,
        "interval_seconds": config.interval_seconds, "observed_at": observed_at,
        "signature": hmac_hex(&config.node_signing_key, &message)
    });
    client
        .post(&config.register_url)
        .bearer_auth(&config.api_token)
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("probe registration request failed: {error}"))?
        .error_for_status()
        .map_err(|error| format!("probe registration rejected: {error}"))?;
    Ok(())
}

async fn connect_tenant(
    config: &CanaryConfig,
    tenant: &str,
) -> Result<tokio_postgres::Client, String> {
    let password = env::var(password_env_name(tenant))
        .map_err(|_| format!("missing password for tenant {tenant}"))?;
    let token = tenant_token(config, tenant);
    let user = env::var(user_env_name(tenant)).unwrap_or_else(|_| default_user_name(tenant));
    if !valid_identifier(&user) {
        return Err(format!(
            "invalid database username configured for tenant {tenant}"
        ));
    }
    let options = format!("-c proxima_tenant_token={token}");
    let mut pg = PgConfig::new();
    pg.host(config.proxy.ip().to_string())
        .port(config.proxy.port())
        .user(&user)
        .password(&password)
        .dbname(&config.database)
        .options(&options)
        .connect_timeout(Duration::from_secs(5));
    let (client, connection) = pg
        .connect(NoTls)
        .await
        .map_err(|error| format!("tenant {tenant} could not connect through Proxima: {error}"))?;
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            tracing::debug!(%error, "tenant-canary PostgreSQL connection closed");
        }
    });
    Ok(client)
}

async fn execute_scenario(config: &CanaryConfig) -> (String, Value) {
    let mut checks = Vec::new();
    let mut failed = false;
    let mut review = false;
    let query = format!(
        "SELECT count(*)::bigint FROM {}.{} WHERE {} = $1",
        config.schema, config.table, config.tenant_column
    );

    // Reuse one authenticated session and prepared statement per tenant.
    // A 10-tenant matrix therefore uses at most 10 connections per run,
    // rather than opening a new connection for each of the 100 checks.
    for tenant in &config.tenants {
        let client = match connect_tenant(config, tenant).await {
            Ok(client) => client,
            Err(error) => {
                return (
                    "error".into(),
                    json!({"scenario_id":SCENARIO_ID,"checks":checks,"error":error}),
                );
            }
        };
        let statement = match client.prepare(&query).await {
            Ok(statement) => statement,
            Err(error) => {
                return (
                    "error".into(),
                    json!({"scenario_id":SCENARIO_ID,"checks":checks,"error":format!("tenant {tenant} could not prepare the canary query: {error}")}),
                );
            }
        };

        match client.query_one(&statement, &[&tenant.as_str()]).await {
            Ok(row) => {
                let count: i64 = row.get(0);
                checks.push(json!({"tenant":tenant,"target":tenant,"kind":"own_canary_visible","expected":"positive","observed":count}));
                if count == 0 {
                    review = true;
                }
            }
            Err(error) => {
                return (
                    "error".into(),
                    json!({"scenario_id":SCENARIO_ID,"checks":checks,"error":format!("tenant {tenant} own-canary query failed: {error}")}),
                );
            }
        }

        for target in config.tenants.iter().filter(|target| *target != tenant) {
            match client.query_one(&statement, &[&target.as_str()]).await {
                Ok(row) => {
                    let count: i64 = row.get(0);
                    checks.push(json!({"tenant":tenant,"target":target,"kind":"cross_tenant_canary_hidden","expected":0,"observed":count}));
                    if count > 0 {
                        failed = true;
                    }
                }
                Err(error) => {
                    return (
                        "error".into(),
                        json!({"scenario_id":SCENARIO_ID,"checks":checks,"error":format!("tenant {tenant} cross-tenant query for {target} failed: {error}")}),
                    );
                }
            }
        }
    }
    let status = if failed {
        "fail"
    } else if review {
        "review"
    } else {
        "pass"
    };
    (
        status.into(),
        json!({
            "scenario_id":SCENARIO_ID,
            "checks":checks,
            "check_count":checks.len(),
            "summary":if failed {"Cross-tenant canary data was visible to another tenant."} else if review {"At least one tenant could not see its own canary record; fixture or policy requires review."} else {"All own-tenant and cross-tenant canary checks matched expectations."}
        }),
    )
}

async fn report(
    config: &CanaryConfig,
    client: &reqwest::Client,
    status: &str,
    evidence: Value,
) -> Result<(), String> {
    let observed_at = now_unix();
    let evidence_json = evidence.to_string();
    let evidence_hash = hex::encode(Sha256::digest(evidence_json.as_bytes()));
    let message = format!(
        "report|{}|{}|{}|{}|{}",
        config.node_id, SCENARIO_ID, status, observed_at, evidence_hash
    );
    let body = json!({
        "node_id":config.node_id,"scenario_id":SCENARIO_ID,"status":status,
        "observed_at":observed_at,"evidence":evidence,"evidence_hash":evidence_hash,
        "signature":hmac_hex(&config.node_signing_key,&message)
    });
    client
        .post(&config.report_url)
        .bearer_auth(&config.api_token)
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("probe report request failed: {error}"))?
        .error_for_status()
        .map_err(|error| format!("probe report rejected: {error}"))?;
    Ok(())
}

pub async fn run_if_enabled(proxy: SocketAddr) {
    let config = match CanaryConfig::from_env(proxy) {
        Ok(Some(config)) => config,
        Ok(None) => return,
        Err(error) => {
            tracing::error!(%error, "tenant canary is enabled but configuration is invalid");
            return;
        }
    };
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            tracing::error!(%error, "tenant canary HTTP client unavailable");
            return;
        }
    };
    loop {
        match register(&config, &client).await {
            Ok(()) => break,
            Err(error) => {
                tracing::error!(%error, "tenant canary registration failed; retrying in 30 seconds");
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        }
    }
    tracing::info!(
        scenario = SCENARIO_ID,
        interval_seconds = config.interval_seconds,
        "tenant canary runner registered"
    );
    let mut ticker = interval(Duration::from_secs(config.interval_seconds));
    loop {
        ticker.tick().await;
        let (status, evidence) = execute_scenario(&config).await;
        if status == "fail" {
            tracing::error!(
                scenario = SCENARIO_ID,
                "tenant canary detected cross-tenant visibility"
            );
        } else if status == "pass" {
            tracing::info!(scenario = SCENARIO_ID, "tenant canary scenario passed");
        } else {
            tracing::warn!(
                scenario = SCENARIO_ID,
                status,
                "tenant canary requires attention"
            );
        }
        if let Err(error) = report(&config, &client, &status, evidence).await {
            tracing::error!(%error, scenario = SCENARIO_ID, "tenant canary outcome could not be delivered");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_plane_urls_require_https_except_for_explicit_local_development() {
        assert!(control_plane_url_allowed(
            "https://control.example.com/api/v1/security/events",
            false
        ));
        assert!(control_plane_url_allowed(
            "http://127.0.0.1:8080/api/v1/security/events",
            false
        ));
        assert!(control_plane_url_allowed(
            "http://localhost:8080/api/v1/security/events",
            false
        ));
        assert!(!control_plane_url_allowed(
            "http://proxima-control:8080/api/v1/security/events",
            false
        ));
        assert!(control_plane_url_allowed(
            "http://proxima-control:8080/api/v1/security/events",
            true
        ));
        assert!(!control_plane_url_allowed(
            "ftp://control.example.com/events",
            true
        ));
    }

    #[test]
    fn identifier_validation_accepts_only_simple_sql_identifiers() {
        for valid in ["records", "_records2", "proxima_security"] {
            assert!(valid_identifier(valid));
        }
        for invalid in [
            "",
            "records; DROP TABLE users",
            "schema.records",
            "x--",
            "2records",
        ] {
            assert!(!valid_identifier(invalid));
        }
    }

    #[test]
    fn tenant_user_environment_name_is_deterministic() {
        assert_eq!(
            user_env_name("tenant_a"),
            "PROXIMA_TENANT_CANARY_USER_TENANT_A"
        );
        assert_eq!(
            user_env_name("tenant-a"),
            "PROXIMA_TENANT_CANARY_USER_TENANT_A"
        );
    }

    #[test]
    fn tenant_default_user_name_is_safe_and_deterministic() {
        assert_eq!(default_user_name("tenant_a"), "proxima_tenant_tenant_a");
        assert_eq!(default_user_name("tenant-a"), "proxima_tenant_tenant_a");
        assert!(valid_identifier(&default_user_name("tenant-a")));
    }

    #[test]
    fn tenant_password_environment_name_is_deterministic() {
        assert_eq!(
            password_env_name("tenant_a"),
            "PROXIMA_TENANT_CANARY_PASSWORD_TENANT_A"
        );
        assert_eq!(
            password_env_name("tenant-a"),
            "PROXIMA_TENANT_CANARY_PASSWORD_TENANT_A"
        );
    }

    #[test]
    fn report_signature_binds_status_and_evidence_hash() {
        let key = [7u8; 32];
        let hash = "ab".repeat(32);
        let signed = format!("report|node|scenario|pass|123|{hash}");
        let signature = hmac_hex(&key, &signed);
        assert_eq!(signature.len(), 64);
        assert_ne!(
            signature,
            hmac_hex(&key, &format!("report|node|scenario|fail|123|{hash}"))
        );
    }
}
