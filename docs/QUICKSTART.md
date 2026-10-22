# Quickstart: run Proxima and query PostgreSQL

This walkthrough starts a real PostgreSQL database and the Engine, sends a SQL query through the Engine, and checks health and metrics. You do not need a hosted account or a pre-existing database.

## Requirements

- Git
- Docker Desktop (Windows/macOS) or Docker Engine + Docker Compose (Linux)
- For the optional native build: stable Rust and Cargo
- For host-side examples: `curl` and `psql`; the Compose walkthrough runs `psql` inside the database container

## 1. Start the stack

```bash
git clone https://github.com/Agata-Proxima/proxima-engine.git
cd proxima-engine
docker compose -f compose.dev.yaml up --build -d
```

PowerShell users can run the same Docker command. If you want to edit local settings first, copy the example environment file with `Copy-Item .env.example .env` in PowerShell or `cp .env.example .env` in Bash.

This development profile starts:
- PostgreSQL on a private Compose network.
- Proxima on port 6432, published to host loopback only.
- The local status page on `127.0.0.1:9080`.
- Health and Prometheus metrics on `127.0.0.1:9090`.

The database password is `local-only-change-me`. It is for local development only and must never be reused outside this profile.

## 2. Confirm the Engine is up

```bash
curl -fsS http://127.0.0.1:9090/healthz
curl -fsS http://127.0.0.1:9080/health
curl -fsS http://127.0.0.1:9080/api/status
```

The health routes return `ok`. The JSON status route reports whether tenant-token enforcement and TLS are configured, plus the listener/upstream settings and connection limit. Do not publish the dashboard or metrics listener to an untrusted network.

In Windows PowerShell, use `Invoke-RestMethod http://127.0.0.1:9090/healthz`, `Invoke-RestMethod http://127.0.0.1:9080/health`, and `Invoke-RestMethod http://127.0.0.1:9080/api/status` instead of the Unix-style `curl -fsS` commands.

## 3. Create a table in PostgreSQL

This command connects directly to the local database to create test data:

```bash
docker compose -f compose.dev.yaml exec postgres psql -U proxima -d appdb -c "CREATE TABLE IF NOT EXISTS quickstart_messages (message text NOT NULL); TRUNCATE quickstart_messages; INSERT INTO quickstart_messages VALUES ('hello through Proxima');"
```

## 4. Read that data through Proxima

Now the client connects to the Engine, not directly to PostgreSQL:

```bash
docker compose -f compose.dev.yaml exec postgres psql "host=engine port=6432 dbname=appdb user=proxima password=local-only-change-me sslmode=disable" -c "SELECT * FROM quickstart_messages;"
```

Expected result: one row containing `hello through Proxima`. You have verified a real PostgreSQL query through the proxy.

## 5. Inspect metrics

```bash
curl -fsS http://127.0.0.1:9090/metrics
```

Look for names such as `proxima_engine_connections_accepted_total`, `proxima_engine_active_connections`, and `proxima_engine_processed_queries_total`. Metrics are process-local counters; restarting the Engine resets them.

## 6. Stop the stack

```bash
docker compose -f compose.dev.yaml down
```

The named volumes remain so the local database can be reused. To permanently delete this disposable development data, use `docker compose -f compose.dev.yaml down -v`. Do not use `-v` if you need the data.

## What this quickstart proves — and what it does not

It proves that the container builds, PostgreSQL starts, the Engine accepts a PostgreSQL connection, the query reaches the database, and the health/metrics listeners respond.

The default Compose profile intentionally leaves tenant-token enforcement off so a first connection is simple. **It does not prove tenant isolation, TLS enforcement, production readiness, or compatibility with every driver.** Continue with [Tenant isolation](TENANT_ISOLATION.md), [Security](SECURITY.md), and [Deployment](DEPLOYMENT.md) before handling sensitive workloads.

## Run natively without Docker

If PostgreSQL is already available at `127.0.0.1:5432`:

```bash
cargo test -p proxima-engine
cargo build -p proxima-engine --release
export PROXIMA_UPSTREAM_ADDR=127.0.0.1:5432
export PROXIMA_LISTEN_ADDR=127.0.0.1:6432
cargo run -p proxima-engine
```

On PowerShell, set the two variables with `$env:PROXIMA_UPSTREAM_ADDR = "127.0.0.1:5432"` and `$env:PROXIMA_LISTEN_ADDR = "127.0.0.1:6432"`, then run `cargo run -p proxima-engine`.

Connect your client to port 6432. The Engine reads configuration from its process environment; see [Configuration](CONFIGURATION.md).


## If something fails

| Symptom | First check | Next guide |
|---|---|---|
| Compose cannot build | Docker is running and the first build can download dependencies | [Deployment](DEPLOYMENT.md) |
| Health endpoint refuses connection | Inspect `docker compose -f compose.dev.yaml ps` and container logs | [Operations](OPERATIONS.md) |
| Health succeeds but SQL fails | Confirm the upstream database is ready and the proxy host/port is correct | [API reference](API.md) |
| Query connects but tenant rows are wrong | Stop sensitive testing; review role grants and RLS policies | [Tenant isolation](TENANT_ISOLATION.md) |
| Enrollment fails | Check token expiry, node identity persistence, and variable names without sharing secret values | [Node enrollment](NODE_ENROLLMENT.md) |
| You need a hosted account or plan | Use the official hosted workflow | [Cloud boundary](CLOUD_CONTROL_PLANE.md) · [Billing](BILLING.md) |

## Next step checklist

- [ ] Local SQL query through the proxy succeeds.
- [ ] Production credentials replace every demo credential.
- [ ] Tenant token signing occurs only in a trusted backend.
- [ ] PostgreSQL roles are non-superuser and least-privileged.
- [ ] RLS policies are tested with both permitted and denied tenant access.
- [ ] TLS verification is enabled for production network paths.
- [ ] Status/metrics endpoints are not publicly exposed without an authenticated boundary.
- [ ] Backup, recovery, monitoring, and secret rotation procedures are documented.

A checklist is a reminder, not evidence that a control is configured. Record the actual test results for your deployment.
