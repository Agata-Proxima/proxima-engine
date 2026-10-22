# Configuration reference

The engine reads configuration from environment variables. Defaults and validation behavior are defined in [config.rs](../crates/proxima-engine/src/config.rs). Values shown as defaults apply when the variable is unset.

| Variable | Default | Purpose |
| --- | --- | --- |
| PROXIMA_LISTEN_ADDR | 127.0.0.1:6432 | Client listener address |
| PROXIMA_UPSTREAM_ADDR | 127.0.0.1:5432 | Upstream database address |
| PROXIMA_TENANT_SIGNING_KEY | Unset | Tenant-token signing key; if set, must be at least 32 bytes |
| PROXIMA_TENANT_ROLE_PREFIX | proxima_tenant_ | Prefix for tenant database roles; ASCII letters, digits, and underscores only |
| PROXIMA_UPSTREAM_CONNECT_TIMEOUT_MS | 10000 | Upstream connection timeout in milliseconds |
| PROXIMA_MAX_CONNECTIONS | 1024 | Maximum configured connections; must be greater than zero |
| PROXIMA_TLS_CERT_FILE | Unset | Client-facing TLS certificate path |
| PROXIMA_TLS_KEY_FILE | Unset | Client-facing TLS private key path; must be paired with the certificate |
| PROXIMA_TLS_REQUIRE_CLIENT | false | Require client TLS |
| PROXIMA_TLS_HANDSHAKE_TIMEOUT_MS | 10000 | Client-side TLS handshake timeout |
| PROXIMA_UPSTREAM_TLS_MODE | disable | Upstream TLS mode: disable or verify-full |
| PROXIMA_UPSTREAM_TLS_CA_FILE | Unset | CA file required when upstream TLS is enabled |
| PROXIMA_UPSTREAM_TLS_SERVER_NAME | Unset | Required server name for verify-full |
| PROXIMA_UPSTREAM_TLS_HANDSHAKE_TIMEOUT_MS | 10000 | Upstream TLS handshake timeout |
| PROXIMA_DASHBOARD_ENABLED | true | Enable the embedded status dashboard |
| PROXIMA_DASHBOARD_LISTEN_ADDR | 127.0.0.1:9080 | Embedded status page address; the page has no built-in authentication |
| PROXIMA_METRICS_LISTEN_ADDR | 127.0.0.1:9090 | Metrics and health listener address; serves `/metrics`, `/health`, and `/healthz` |\n| PROXIMA_NODE_ID | Unset | Node identifier supplied by a compatible control plane or loaded from saved identity |\n| PROXIMA_NODE_ENROLLMENT_URL | Unset | Compatible control-plane URL for one-time node enrollment |\n| PROXIMA_NODE_ENROLLMENT_TOKEN | Unset | One-time bootstrap token; consumed during enrollment |\n| PROXIMA_NODE_IDENTITY_PATH | Platform default | Persistent node identity file; restrict file access |\n| PROXIMA_NODE_SIGNING_KEY_HEX | Unset | 32-byte node signing credential encoded as hexadecimal; normally loaded from identity file |\n| PROXIMA_ATTESTATION_LEDGER_PATH | data/proxima-attestations.jsonl | Local hash-linked attestation ledger path |\n| PROXIMA_ATTESTATION_SIGNING_KEY | Unset | Alternative attestation signing key; at least 32 bytes |\n| PROXIMA_ORGANIZATION_ID | unconfigured | Organization label recorded in local attestation evidence |\n| PROXIMA_ATTESTATION_P99_OVERHEAD_MS | Unset | Operator-supplied non-negative latency overhead value required to enable hourly attestation |\n| PROXIMA_ATTESTATION_INGEST_URL | Unset | Optional HTTPS (or allowed local development HTTP) receiver for signed evidence |\n| PROXIMA_ATTESTATION_INGEST_TOKEN | Unset | Optional bearer credential for the configured evidence receiver; expected prefix `aga_` |\n| PROXIMA_SECURITY_EVENT_INGEST_URL | Unset | Optional compatible security-event receiver URL |\n| PROXIMA_SECURITY_EVENT_INGEST_TOKEN | Unset | Optional bearer credential for security-event delivery; expected prefix `aga_` |\n| PROXIMA_SECURITY_BREAKER | fail-closed | Set to `off` to disable peer-scoped violation breaker; keep enabled in normal use |\n| PROXIMA_BREAKER_FAILURE_THRESHOLD | 5 | Violations in the observation window before a peer is blocked |\n| PROXIMA_BREAKER_WINDOW_SECONDS | 60 | Security-breaker observation window |\n| PROXIMA_BREAKER_COOLDOWN_SECONDS | 300 | Security-breaker block duration |\n| PROXIMA_BREAK_GLASS_REPLAY_DIR | data/proxima-break-glass-replays | Persistent directory used to reject replayed emergency tokens |

Boolean values accept 1, true, yes, on, 0, false, no, and off (case-insensitive). Timeout values must be positive integers.

## Example local setup

This example uses local loopback addresses. It is not a production deployment recipe.

- Set PROXIMA_LISTEN_ADDR to 127.0.0.1:6432.
- Set PROXIMA_UPSTREAM_ADDR to 127.0.0.1:5432.
- Set PROXIMA_DASHBOARD_ENABLED to true.
- Set PROXIMA_DASHBOARD_LISTEN_ADDR to 127.0.0.1:9080.
- Run cargo run -p proxima-engine.

For TLS, configure certificate/key paths and the upstream CA/server name according to the [security guidance](SECURITY.md). Never commit private keys or signing keys.

## Attestation configuration

Attestation settings are read in [attestation.rs](../crates/proxima-engine/src/attestation.rs):

- PROXIMA_ATTESTATION_LEDGER_PATH — local ledger path (default: data/proxima-attestations.jsonl)
- PROXIMA_NODE_SIGNING_KEY_HEX — 32-byte key represented as hexadecimal
- PROXIMA_ATTESTATION_SIGNING_KEY — alternative signing key, at least 32 bytes
- PROXIMA_ORGANIZATION_ID — organization label included in evidence
- PROXIMA_NODE_ID — node label included in evidence

Protect the signing key and ledger as sensitive security assets. Consult the source before depending on the attestation output for compliance or audit requirements.
