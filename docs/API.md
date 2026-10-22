# Network and API reference

Proxima Engine is primarily a PostgreSQL wire-protocol proxy. It is **not** a general-purpose REST API server, and this repository does not expose the private Agata Proxima control-plane API or customer dashboard API.

## Network surfaces

| Surface | Default | Purpose |
| --- | --- | --- |
| PostgreSQL proxy | `127.0.0.1:6432` | Accepts PostgreSQL client connections and forwards sessions to the configured upstream database. |
| Upstream PostgreSQL | `127.0.0.1:5432` | Database target configured by `PROXIMA_UPSTREAM_ADDR`. |
| Local status dashboard | `127.0.0.1:9080` | Optional embedded dashboard and read-only local status routes. |
| Metrics listener | Configured by the engine's metrics listener settings | Exposes Prometheus-formatted engine metrics. Check the current [metrics implementation](../crates/proxima-engine/src/metrics.rs) and startup logs for the configured address. |

Defaults are intended for local development. Do not assume a listener is authenticated merely because it is local in a development setup; bind to loopback and restrict access at the network layer.

## PostgreSQL connection

Clients use the PostgreSQL protocol and connect to the engine's configured listen address. Example:

```bash
psql "host=127.0.0.1 port=6432 dbname=postgres user=YOUR_DATABASE_USER"
```

The target database must already exist and accept the supplied database credentials. The engine does not bundle PostgreSQL.

## Local dashboard HTTP routes

The embedded dashboard currently implements these routes in [dashboard.rs](../crates/proxima-engine/src/dashboard.rs):

| Method | Path | Response |
| --- | --- | --- |
| `GET` | `/` | Embedded HTML dashboard. |
| `GET` | `/logo.svg` | Embedded logo asset. |
| `GET` | `/health` | Plain-text `ok` health response. |
| `GET` | `/api/status` | JSON with tenant-enforcement, TLS, listen/upstream address, and connection-limit status. |
| Other paths | — | `404 Not Found`. |

The current embedded handler is intentionally small. It is not a customer-management API, does not create or enroll nodes through HTTP, and does not implement a webhook-management API.

## Prometheus metrics

The engine exposes Prometheus-formatted operational metrics through its metrics listener. Metric names and descriptions are defined in [metrics.rs](../crates/proxima-engine/src/metrics.rs). They include connection counts, sessions, processed query messages, bytes forwarded, security-breaker activity, and query latency observations.

Use the configured metrics address from startup logs and configure your Prometheus scrape target accordingly. Do not expose metrics publicly without appropriate network restrictions.

## Node enrollment, hosted API keys, and webhooks

Node enrollment, organization/customer administration, hosted API keys, and hosted webhooks belong to the private hosted Agata Proxima service and are not implemented as public REST endpoints by this engine repository. Do not invent endpoint URLs or paste hosted credentials into engine configuration without following the hosted product's official instructions.

If you need the hosted platform, go to [agataproxima.com](https://agataproxima.com), sign in to your account, and use the developer documentation provided there.

## Contract and compatibility

Treat the implementation as the source of truth. Before integrating:

1. Pin a release or commit.
2. Check the route behavior in `crates/proxima-engine/src/dashboard.rs`.
3. Check runtime settings in `crates/proxima-engine/src/config.rs`.
4. Run `cargo test -p proxima-engine`.
5. Validate TLS, network exposure, failure handling, and metrics in your target environment.
