# Demonstrate tenant isolation: four reproducible scenarios

This is a **test plan with expected results**, not a claim that the tests have been executed. Use synthetic data in the disposable local Docker environment.

## Setup

Follow [Quickstart](QUICKSTART.md), then sections 1–3 of [Tenant isolation](TENANT_ISOLATION.md). These steps create tenant A/B roles and RLS policies, enable `PROXIMA_TENANT_SIGNING_KEY`, and create a short-lived signed `TENANT_TOKEN` for `tenant_a`. Run the commands below in that shell.

## 1. Valid tenant context

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id, secret FROM tenant_demo.records;"
```

**Expected:** only `tenant_a | A-secret`.

## 2. Missing tenant context

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS='' \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id FROM tenant_demo.records;"
```

**Expected:** connection/startup rejected when token verification is enabled. If the query succeeds, stop and check the Engine configuration and proxy endpoint.

## 3. Attempted cross-tenant access

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT count(*) FROM tenant_demo.records WHERE tenant_id='tenant_b';"
```

**Expected:** `0`.

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "INSERT INTO tenant_demo.records (tenant_id,secret) VALUES ('tenant_b','denied');"
```

**Expected:** PostgreSQL RLS rejects the write.

## 4. Tenant context changes between connections

Using the token-generation command in [Tenant isolation](TENANT_ISOLATION.md), generate a **new** token with `tenant="tenant_b"` and save it as `TENANT_B_TOKEN`. Connect using Tenant B's password and `PGOPTIONS="-c proxima_tenant_token=$TENANT_B_TOKEN"`; repeat the first SELECT.

**Expected:** only `tenant_b | B-secret`. Repeat Tenant A's SELECT on another connection and expect only A's row. This does **not** prove request-level isolation on a shared pooled connection; test pooling separately.

## Evidence checklist

Capture actual outputs, Engine commit SHA, date, PostgreSQL version, and non-sensitive configuration status. Redact signing keys, passwords, tokens, private addresses, and real customer data. Distinguish expected from observed results.

**Security boundary:** the Engine validates signed tenant context and maps it to a PostgreSQL role; PostgreSQL grants and RLS enforce row-level isolation. This open-source walkthrough does not claim hosted verification or audit records were generated. It does not replace application authorization, TLS, secure token issuance, restricted direct database access, or connection-pool testing.

See [Security](SECURITY.md) for production limitations.
