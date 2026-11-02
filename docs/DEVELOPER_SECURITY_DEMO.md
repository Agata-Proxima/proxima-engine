# Developer security walkthrough: tenant access scenarios

This walkthrough provides repeatable commands for checking tenant authentication and PostgreSQL row boundaries with synthetic data. The repository also includes recorded execution evidence for tenant isolation and PgBouncer transaction pooling.

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

## Recorded PgBouncer transaction-pooling evidence

The repository records a local run using PgBouncer 1.24.1 in transaction mode and PostgreSQL 17. Five sequential connections for each tenant reused that tenant's backend PID. Tenant A and Tenant B used distinct PostgreSQL roles and distinct observed backend PIDs.

See [PgBouncer transaction-pooling evidence](../evidence/pgbouncer-transaction-pooling.txt) for the environment, SQL checks, backend PID observations, and result summary.

The recorded evidence demonstrates per-tenant role separation and backend reuse within each tenant role. It records separate backend identities across the two tenant roles.

## Evidence and safe operation

For reproducible results, record the Engine commit, PostgreSQL version, exact commands, exit status, and sanitized output. Use synthetic data and redact signing keys, passwords, tokens, private addresses, and customer information.

The Engine validates signed tenant context and maps the verified tenant to a PostgreSQL role. PostgreSQL grants and RLS policies enforce the row boundary. The trusted application backend remains responsible for authenticating users and authorizing tenant membership before issuing tenant tokens.

See [Security](SECURITY.md), [Deployment](DEPLOYMENT.md), and [Verification](VERIFICATION.md).
