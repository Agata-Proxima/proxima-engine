# Proxima Engine API, health, and metrics

Proxima Engine is a PostgreSQL wire-protocol proxy with a local HTTP status and observability surface. Applications send SQL through the PostgreSQL endpoint; operators use the HTTP routes for health, status, and Prometheus metrics.

## Local HTTP routes

| Listener | Default address | Route | Response |
|---|---|---|---|
| Embedded status page | `127.0.0.1:9080` | `GET /` | Engine status page |
| Embedded status page | `127.0.0.1:9080` | `GET /logo.svg` | Agata Proxima logo |
| Embedded status page | `127.0.0.1:9080` | `GET /health` | Plain-text `ok` |
| Embedded status page | `127.0.0.1:9080` | `GET /api/status` | JSON runtime/configuration summary |
| Metrics listener | `127.0.0.1:9090` | `GET /metrics` | Prometheus exposition |
| Metrics listener | `127.0.0.1:9090` | `GET /health`, `GET /healthz` | Plain-text `ok` |

The default listeners bind to loopback. The embedded status server has no built-in user authentication; keep it on loopback or behind a separately authenticated private access layer. The status JSON includes listener/upstream addresses and whether tenant enforcement and TLS are configured.

## Verify a running Engine

```bash
curl -i http://127.0.0.1:9090/healthz
curl -i http://127.0.0.1:9080/health
curl -i http://127.0.0.1:9080/api/status
curl -i http://127.0.0.1:9090/metrics
```

Health routes return HTTP 200 with body `ok`. Unknown routes return HTTP 404. Follow a health check with a SQL query through port 6432 to verify the PostgreSQL protocol path.

## PostgreSQL endpoint

Applications use their existing PostgreSQL driver to connect to the Engine listener, normally port 6432. The Engine connects upstream to `PROXIMA_UPSTREAM_ADDR`.

```bash
psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima password=YOUR_DATABASE_PASSWORD sslmode=disable"
```

Use `sslmode=disable` only for the loopback development quickstart. For deployed connections, configure TLS and verify certificates as described in [Deployment](DEPLOYMENT.md).

## Prometheus metrics

The metrics listener serves Prometheus text format. Available series include:

- `proxima_engine_active_connections`
- `proxima_engine_connections_accepted_total`
- `proxima_engine_connections_rejected_total`
- `proxima_engine_sessions_established_total`
- `proxima_engine_sessions_failed_total`
- `proxima_engine_processed_queries_total`
- `proxima_engine_authenticated_tenant_sessions_total`
- `proxima_engine_security_violations_blocked_total`
- `proxima_engine_security_breaker_trips_total`
- `proxima_engine_client_bytes_total`
- `proxima_engine_database_bytes_total`
- `proxima_engine_session_duration_seconds_total`
- `proxima_engine_query_roundtrip_latency_seconds_bucket`
- `proxima_engine_query_roundtrip_latency_seconds_count`

Counters and histograms are process-local and reset when the Engine restarts. Scrape them from a private monitoring network; use the attestation ledger for configured signed evidence.

## Node enrollment contract

When enrollment variables are configured, the Engine sends an outbound `POST /api/v1/nodes/enroll` request to the configured control-plane URL with JSON fields `node_id` and `enrollment_token`. It validates the returned identity and persists it locally.

This is an outbound enrollment request, not a route served by the public Engine. Hosted node registration and one-time token issuance are handled by a compatible control plane. See [Node enrollment](NODE_ENROLLMENT.md).

## Tenant context

When `PROXIMA_TENANT_SIGNING_KEY` is configured, the Engine requires a valid signed tenant token during PostgreSQL startup. libpq-compatible clients pass it in the startup `options` value:

```text
-c proxima_tenant_token=<signed-token>
```

The Engine removes this private option before forwarding startup to PostgreSQL, then rewrites the upstream username to `<PROXIMA_TENANT_ROLE_PREFIX><tenant_id>` (hyphens in the tenant ID become underscores). PostgreSQL grants and RLS policies enforce the role's data access. Follow [Tenant isolation](TENANT_ISOLATION.md) for runnable SQL and test results.

## Select the correct interface

- **PostgreSQL endpoint:** application SQL and database sessions.
- **Health routes:** process-level HTTP health.
- **Status page and `/api/status`:** private runtime diagnostics.
- **`/metrics`:** Prometheus telemetry for trusted monitoring.
- **Agata Proxima Cloud:** hosted accounts, organizations, teams, plans, billing, and hosted node management.

The Engine's local HTTP surface is intentionally focused on status and observability. Hosted account and management workflows belong to the separate Cloud product. See [Cloud control-plane boundary](CLOUD_CONTROL_PLANE.md).

For route failures, verify the listener, bind address, caller network, and firewall policy. See [Operations](OPERATIONS.md) and [Configuration](CONFIGURATION.md).
