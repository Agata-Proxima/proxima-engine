# Quickstart

This guide builds Proxima Engine and connects a PostgreSQL client through its local TCP proxy. It does not provision PostgreSQL or create an account in the hosted Agata Proxima service.

## 1. Prerequisites

- Stable Rust toolchain with `cargo`
- A PostgreSQL server that is already running and reachable
- `psql` or another PostgreSQL client
- Network access to download Rust dependencies

## 2. Clone, test, and build

```bash
git clone https://github.com/Agata-Proxima/proxima-engine.git
cd proxima-engine
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

## 3. Configure a local proxy

The defaults bind the PostgreSQL proxy to `127.0.0.1:6432` and target PostgreSQL at `127.0.0.1:5432`. Set these explicitly if your database uses different addresses:

```bash
export PROXIMA_LISTEN_ADDR=127.0.0.1:6432
export PROXIMA_UPSTREAM_ADDR=127.0.0.1:5432
cargo run -p proxima-engine
```

Keep the process running. The database target must accept the credentials you use in the client.

## 4. Connect through the engine

In a second terminal, use your own database name and user:

```bash
psql "host=127.0.0.1 port=6432 dbname=postgres user=YOUR_DATABASE_USER"
```

Run a harmless query such as `SELECT version();`. If the connection fails, confirm PostgreSQL is reachable directly at the configured upstream address, the database and user exist, and the engine logs do not show a configuration or connection error.

## 5. Inspect local status

With the default local dashboard enabled, open [http://127.0.0.1:9080/](http://127.0.0.1:9080/) in a browser. Its health route is [http://127.0.0.1:9080/health](http://127.0.0.1:9080/health), and its status route is [http://127.0.0.1:9080/api/status](http://127.0.0.1:9080/api/status).

These are local engine status routes, not the hosted Agata Proxima customer dashboard or its control-plane API. Keep the dashboard bound to loopback unless you have added and reviewed appropriate network access controls.

## 6. Configure security before shared use

The quickstart is deliberately local. Before connecting other machines or handling real workloads:

- Review [Configuration](CONFIGURATION.md) and [Security](SECURITY.md).
- Configure client TLS and upstream TLS where required by your environment.
- Use a strong tenant signing key when tenant-token verification is part of your deployment.
- Restrict network access to the proxy, dashboard, and metrics listener.
- Store secrets in a secret manager or protected runtime environment, never in source control.
- Test failure, restart, and recovery behavior in your own environment.

## Troubleshooting

| Symptom | Check |
| --- | --- |
| Engine exits at startup | Read the validation error; verify address formats and paired TLS certificate/key settings. |
| Connection refused | Confirm PostgreSQL is running and reachable at `PROXIMA_UPSTREAM_ADDR`. |
| Client cannot connect to proxy | Confirm the engine is running and your client targets `PROXIMA_LISTEN_ADDR`. |
| TLS configuration error | Follow the requirements in [Configuration](CONFIGURATION.md). |
| Tenant verification rejects a session | Check the signing-key configuration and the token's signature, expiry, and tenant context. |

For a reproducible engine defect, open an issue with the steps, expected result, actual result, and sanitized logs. Never post credentials, signing keys, or customer data.
