# Tenant isolation: PostgreSQL roles, signed context, and RLS

This guide configures two tenant roles, enables PostgreSQL Row-Level Security (RLS), signs tenant context, and sends each tenant through Proxima Engine. It finishes with positive and negative SQL checks so the database boundary is visible and reproducible.

Run the commands against the disposable local database from the [Quickstart](QUICKSTART.md).

## 1. Create two tenant roles and an RLS-protected table

Run from the repository root:

```bash
docker compose -f compose.dev.yaml exec -T postgres psql -U proxima -d appdb <<'SQL'
DROP SCHEMA IF EXISTS tenant_demo CASCADE;
DROP ROLE IF EXISTS proxima_tenant_tenant_a;
DROP ROLE IF EXISTS proxima_tenant_tenant_b;

CREATE ROLE proxima_tenant_tenant_a LOGIN PASSWORD 'tenant-a-password' NOSUPERUSER NOBYPASSRLS;
CREATE ROLE proxima_tenant_tenant_b LOGIN PASSWORD 'tenant-b-password' NOSUPERUSER NOBYPASSRLS;

CREATE SCHEMA tenant_demo;
CREATE TABLE tenant_demo.records (
  id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  tenant_id text NOT NULL,
  secret text NOT NULL
);

INSERT INTO tenant_demo.records (tenant_id, secret)
VALUES ('tenant_a', 'A-secret'), ('tenant_b', 'B-secret');

ALTER TABLE tenant_demo.records ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_demo.records FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_a_isolation ON tenant_demo.records
  FOR ALL TO proxima_tenant_tenant_a
  USING (tenant_id = 'tenant_a')
  WITH CHECK (tenant_id = 'tenant_a');
CREATE POLICY tenant_b_isolation ON tenant_demo.records
  FOR ALL TO proxima_tenant_tenant_b
  USING (tenant_id = 'tenant_b')
  WITH CHECK (tenant_id = 'tenant_b');

GRANT USAGE ON SCHEMA tenant_demo TO proxima_tenant_tenant_a, proxima_tenant_tenant_b;
GRANT SELECT, INSERT, UPDATE, DELETE ON tenant_demo.records TO proxima_tenant_tenant_a, proxima_tenant_tenant_b;
GRANT USAGE, SELECT ON SEQUENCE tenant_demo.records_id_seq TO proxima_tenant_tenant_a, proxima_tenant_tenant_b;
SQL
```

The roles are ordinary PostgreSQL logins, not superusers, and do not have `BYPASSRLS`. Each role receives a policy that limits its access to its own tenant identifier.

## 2. Enable tenant-token enforcement

Generate a strong local signing key:

```bash
openssl rand -hex 32
```

For the local CLI path, run `python scripts/agata.py init` and copy the literal `tenant_signing_key` from `.proxima-dev/dev-config.json` into the Engine environment as `PROXIMA_TENANT_SIGNING_KEY`. Treat the key as a secret.

For the Compose Engine, export the key before recreating the Engine container:

```bash
export PROXIMA_TENANT_SIGNING_KEY='<same-local-key>'
docker compose -f compose.dev.yaml up -d --build --force-recreate engine
```

The trusted service that signs tenant tokens must use the same key. Keep the signing key on trusted server-side infrastructure; never place it in browser or mobile code.

## 3. Mint short-lived tenant tokens

For Tenant A:

```bash
export PROXIMA_TENANT_SIGNING_KEY='<same-local-key>'
TENANT_TOKEN_A=$(python scripts/agata.py token --tenant tenant_a --ttl 300)
```

For Tenant B:

```bash
TENANT_TOKEN_B=$(python scripts/agata.py token --tenant tenant_b --ttl 300)
```

The token format is:

```text
v1.<tenant_id>.<expiry_unix_seconds>.<hex_hmac_sha256>
```

The HMAC-SHA256 signature covers the exact string `v1.<tenant_id>.<expiry_unix_seconds>`. Tenant IDs may contain ASCII letters, numbers, hyphens, and underscores. The Engine rejects expired, malformed, or incorrectly signed tokens.

In an application, mint tokens only on a trusted server after verifying the authenticated user's tenant membership.

## 4. Connect as Tenant A through Proxima

The Engine derives the upstream PostgreSQL role from the verified tenant ID. The client connects to the proxy as `proxima` and supplies the password for the mapped tenant role:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_A" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id, secret FROM tenant_demo.records;"
```

Expected result: only `tenant_a | A-secret`.

Check the cross-tenant read:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_A" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT count(*) FROM tenant_demo.records WHERE tenant_id = 'tenant_b';"
```

Expected result: `0`.

Check the cross-tenant write:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_A" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "INSERT INTO tenant_demo.records (tenant_id, secret) VALUES ('tenant_b', 'denied');"
```

Expected result: PostgreSQL rejects the insert under the RLS policy.

## 5. Connect as Tenant B

```bash
PGPASSWORD='tenant-b-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN_B" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id, secret FROM tenant_demo.records;"
```

Expected result: only `tenant_b | B-secret`.

## 6. What the tests verify

- A valid signed tenant token is checked during PostgreSQL connection startup.
- The Engine maps the verified tenant ID to the configured tenant-role prefix.
- The private token startup option is removed before the startup packet is forwarded.
- Tenant A and Tenant B receive their own PostgreSQL role and RLS policy.
- Cross-tenant reads return zero rows.
- Cross-tenant writes are rejected by PostgreSQL.
- Missing or invalid tenant context is rejected when `PROXIMA_TENANT_SIGNING_KEY` is configured.

## 7. Verify connection pooling

Run the dedicated [PgBouncer tenant-pool integration test](../scripts/test-pgbouncer-pooling.sh):

```bash
bash scripts/test-pgbouncer-pooling.sh
```

The test starts an isolated Compose project, sends Tenant A and Tenant B through Proxima Engine and PgBouncer transaction pooling, and checks five sequential connections per tenant. Each tenant's connections reuse the same backend PID; the two tenant roles use distinct backend PIDs and separate PgBouncer pools. It also checks cross-tenant read/write denial and missing/invalid token rejection.

The same test runs in the repository's GitHub Actions workflow. The earlier local execution record is available in [PgBouncer transaction-pooling evidence](../evidence/pgbouncer-transaction-pooling.txt).

## Operational requirements

- Use a disposable database with synthetic tenant records for this walkthrough.
- Keep tenant roles non-superuser and without `BYPASSRLS`.
- Apply RLS policies to every tenant-scoped table and every relevant operation.
- Protect the token-signing key and issue tokens only after trusted application authorization.
- Keep direct privileged database paths outside ordinary application traffic.
- Use verified TLS and restricted network access for deployed environments.
- Repeat positive and negative tests after changing roles, grants, policies, drivers, proxy settings, or pool configuration.

See [Security](SECURITY.md), [Deployment](DEPLOYMENT.md), and [Verification](VERIFICATION.md).
