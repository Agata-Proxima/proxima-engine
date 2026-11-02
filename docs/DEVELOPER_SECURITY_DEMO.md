# Developer security walkthrough: tenant access scenarios

This walkthrough provides repeatable commands for checking tenant authentication and PostgreSQL row boundaries with synthetic data. The repository includes an automated PgBouncer transaction-pooling test and recorded execution evidence.

## Setup

Follow [Quickstart](QUICKSTART.md), then complete sections 1–3 of [Tenant isolation](TENANT_ISOLATION.md). The setup creates Tenant A and Tenant B roles, applies RLS policies, enables `PROXIMA_TENANT_SIGNING_KEY`, and generates a short-lived token for Tenant A.

## 1. Valid tenant context

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_A" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id, secret FROM tenant_demo.records;"
```

Expected: only `tenant_a | A-secret`.

## 2. Missing tenant context

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS='' \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id FROM tenant_demo.records;"
```

Expected: connection/startup is rejected while tenant-token enforcement is enabled.

## 3. Cross-tenant read and write

Read attempt:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_A" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT count(*) FROM tenant_demo.records WHERE tenant_id='tenant_b';"
```

Expected: `0`.

Write attempt:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_A" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "INSERT INTO tenant_demo.records (tenant_id,secret) VALUES ('tenant_b','denied');"
```

Expected: PostgreSQL rejects the write under RLS.

## 4. Tenant B context

Generate a Tenant B token using [Tenant isolation](TENANT_ISOLATION.md), connect with Tenant B's database-role password, and repeat the permitted read. Expected result: only `tenant_b | B-secret`.

## Automated PgBouncer transaction-pooling test

Run:

```bash
bash ../scripts/test-pgbouncer-pooling.sh
```

From the repository root, the command is:

```bash
bash scripts/test-pgbouncer-pooling.sh
```

The test starts PostgreSQL, PgBouncer 1.24.1, and Proxima Engine in an isolated Compose project. It opens five sequential client connections for each tenant role and checks that each role reuses its own backend PID while Tenant A and Tenant B retain separate backend PIDs and separate PgBouncer pools.

It also verifies tenant-specific reads, cross-tenant read isolation, RLS write rejection, and missing/invalid token rejection. GitHub Actions runs this test on pull requests and pushes to `main`.

The earlier local execution record is available in [PgBouncer transaction-pooling evidence](../evidence/pgbouncer-transaction-pooling.txt).

## Evidence and safe operation

For reproducible results, record the Engine commit, PostgreSQL and PgBouncer versions, exact commands, exit status, and sanitized output. Use synthetic data and redact signing keys, passwords, tokens, private addresses, and customer information.

The Engine validates signed tenant context and maps the verified tenant to a PostgreSQL role. PostgreSQL grants and RLS policies enforce the row boundary. The trusted application backend remains responsible for authenticating users and authorizing tenant membership before issuing tenant tokens.

See [Security](SECURITY.md), [Deployment](DEPLOYMENT.md), and [Verification](VERIFICATION.md).
