# API, health, metrics, and integration reference

Proxima Engine is a PostgreSQL wire-protocol proxy with a small local HTTP status surface. It is **not** a general-purpose REST API server. This document lists the HTTP routes implemented in this repository.

## Local HTTP surfaces

| Listener | Default | Routes | Purpose |
| --- | --- | --- | --- |
| Embedded status page | `127.0.0.1:9080` | `GET /` | Engine status page with the Agata Proxima logo |
| Embedded status page | `127.0.0.1:9080` | `GET /logo.svg` | Project logo asset |
| Embedded status page | `127.0.0.1:9080` | `GET /health` | Plain-text `ok` |
| Embedded status page | `127.0.0.1:9080` | `GET /api/status` | JSON status/configuration summary |
| Metrics listener | `127.0.0.1:9090` | `GET /metrics` | Prometheus exposition |
| Metrics listener | `127.0.0.1:9090` | `GET /health`, `GET /healthz` | Plain-text `ok` |

The defaults are bound to loopback. The embedded status server does not implement user authentication or authorization. Keep it private; do not publish it to the internet. The status JSON includes listener/upstream addresses and whether tenant enforcement and TLS are configured.

## Verify a running Engine

```bash
curl -i http://127.0.0.1:9090/healthz
curl -i http://127.0.0.1:9080/health
curl -i http://127.0.0.1:9080/api/status
curl -i http://127.0.0.1:9090/metrics
```

A healthy route returns HTTP 200 with body `ok`. An unknown route returns HTTP 404. A successful health response means the listener is responding; it does not prove that the upstream database is reachable or that tenant isolation is correctly configured. Verify an actual database query separately.

## PostgreSQL endpoint

Applications connect using their existing PostgreSQL driver to the Engine listener, normally port 6432. The Engine opens an upstream connection to `PROXIMA_UPSTREAM_ADDR`.

```bash
psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima password=YOUR_DATABASE_PASSWORD sslmode=disable"
```

Use `sslmode=disable` only in the loopback development quickstart. For production, configure TLS and verify certificates according to [Deployment](DEPLOYMENT.md).

## Prometheus metrics

The metrics listener serves Prometheus text format. Useful series include:

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

The counters and histogram are in-memory and reset when the process restarts. Scrape them on a private monitoring network. They are operational telemetry, not a durable audit log.

## Node enrollment contract

When enrollment variables are supplied, the Engine calls `POST /api/v1/nodes/enroll` on the configured control-plane URL with JSON fields `node_id` and `enrollment_token`. It validates the returned identity and persists it locally.

This is an **outbound request made by the Engine**, not an HTTP route served by this Engine repository. Node registration and one-time token issuance require a compatible control plane. The public Engine does not expose `POST /api/v1/nodes`, tenant-management routes, hosted API-key management, or webhook-management routes. See [Node enrollment](NODE_ENROLLMENT.md).

## Tenant context

When `PROXIMA_TENANT_SIGNING_KEY` is configured, the Engine requires a valid signed tenant token during PostgreSQL startup. libpq-compatible clients can pass it in the startup `options` value:

```text
-c proxima_tenant_token=<signed-token>
```

The Engine removes this private option before forwarding startup to PostgreSQL, then rewrites the upstream username to `<PROXIMA_TENANT_ROLE_PREFIX><tenant_id>` (hyphens in the tenant ID become underscores). PostgreSQL role grants and RLS policies remain essential. Follow [Tenant isolation](TENANT_ISOLATION.md) for runnable SQL and token generation.

## HTTP status meanings

| Status | Meaning |
| --- | --- |
| `200` | Health, status, logo, dashboard, or metrics response was served |
| `404` | The requested path is not implemented on that listener |
| No response / connection refused | Listener is disabled, not bound to that interface, process is down, or a network rule blocks access |

The status and metrics listeners do not provide an administrative REST API. Do not invent hosted API endpoints from these local routes.
