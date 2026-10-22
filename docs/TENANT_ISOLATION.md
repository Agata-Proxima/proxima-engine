# Tenant isolation: a runnable example

This tutorial configures PostgreSQL roles and Row-Level Security (RLS), enables Proxima tenant-token verification, and sends a signed token through the PostgreSQL startup options. It is a local learning example, not a production security certification.

Start the [Compose quickstart](QUICKSTART.md) first. These commands use the disposable local database and the local-only password.

## 1. Create two tenant roles and an RLS-protected table

Run this from the repository root:

```bash
docker compose -f compose.dev.yaml exec -T postgres psql -U proxima -d appdb <<'SQL'
DROP SCHEMA IF EXISTS tenant_demo CASCADE;
DROP ROLE IF EXISTS proxima_tenant_tenant_a;
DROP ROLE IF EXISTS proxima_tenant_tenant_b;

CREATE ROLE proxima_tenant_tenant_a LOGIN PASSWORD 'tenant-a-password' NOSUPERUSER NOBYPASSRLS;
CREATE ROLE proxima_tenant_tenant_b LOGIN PASSWORD 'tenant-b-password' NOSUPERUSER NOBYPASSRLS;

CREATE SCHEMA tenant_demo;
CREATE TABLE IF NOT EXISTS tenant_demo.records (
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

For a repeatable run, reset this disposable schema and roles first or change the example names. The roles are ordinary PostgreSQL logins; they are not superusers and cannot bypass RLS. In production, use separate least-privilege provisioning credentials and review grants carefully.

## 2. Enable tenant-token enforcement

Create a strong key. On Linux/macOS:

```bash
openssl rand -hex 32
```

Copy the generated value into `.env` as `PROXIMA_TENANT_SIGNING_KEY=<generated-value>`. Treat it as a secret. Restart the Engine so it reads the value:

```bash
docker compose -f compose.dev.yaml up -d --build --force-recreate engine
```

The same exact key must be used by the trusted service that signs tenant tokens. Do not place the signing key in a browser or mobile app.

## 3. Sign a short-lived tenant token

The token format is:

```text
v1.<tenant_id>.<expiry_unix_seconds>.<hex_hmac_sha256>
```

The HMAC-SHA256 signature covers the exact string `v1.<tenant_id>.<expiry_unix_seconds>`. Tenant IDs may contain ASCII letters, numbers, hyphens, and underscores. The Engine rejects expired, malformed, or incorrectly signed tokens.

For a **local test only**, set the same secret in your shell and generate a five-minute token using Python:

```bash
export PROXIMA_TENANT_SIGNING_KEY='<paste-the-same-local-key>'
TENANT_TOKEN=$(python3 -c 'import os,time,hmac,hashlib; tenant="tenant_a"; expiry=str(int(time.time())+300); payload=f"v1.{tenant}.{expiry}"; print(payload+"."+hmac.new(os.environ["PROXIMA_TENANT_SIGNING_KEY"].encode(),payload.encode(),hashlib.sha256).hexdigest())')
```

In a real application, generate tokens only on a trusted server after checking the authenticated user's organization and tenant membership. Never accept an arbitrary tenant ID directly from an untrusted client.

## 4. Connect as Tenant A through Proxima

The Engine uses the signed tenant ID to change the upstream PostgreSQL startup username to `proxima_tenant_tenant_a`. The client sends that role's password while connecting to the proxy:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "SELECT tenant_id, secret FROM tenant_demo.records;"
```

Expected: only the `tenant_a / A-secret` row is returned. You can also verify the other tenant's row is invisible:

```bash
docker compose -f compose.dev.yaml exec \\
  -e PGPASSWORD=tenant-a-password \\
  -e "PGOPTIONS=-c proxima_tenant_token=$TENANT_TOKEN" \\
  postgres psql "host=engine port=6432 dbname=appdb user=proxima sslmode=disable" \\
  -c "SELECT count(*) FROM tenant_demo.records WHERE tenant_id = 'tenant_b';"
```

Expected count: `0`.

Test the negative case:

```bash
PGPASSWORD='tenant-a-password' PGOPTIONS="-c proxima_tenant_token=$TENANT_TOKEN" \
  psql "host=127.0.0.1 port=6432 dbname=appdb user=proxima sslmode=disable" \
  -c "INSERT INTO tenant_demo.records (tenant_id, secret) VALUES ('tenant_b', 'should be denied');"
```

Expected: PostgreSQL rejects the cross-tenant insert due to the RLS policy.

## 5. What this example demonstrates

- A valid signed tenant token is checked at connection startup.
- The Engine maps the tenant ID to the configured role prefix plus the normalized tenant ID.
- The private token startup option is removed before forwarding the packet.
- PostgreSQL role grants and RLS policies enforce the row boundary.
- A missing or invalid token is rejected when `PROXIMA_TENANT_SIGNING_KEY` is configured.

This example does **not** remove the need to secure the direct PostgreSQL network path, the database administrator credentials, signing-key storage, TLS, backup access, or application authorization. Do not let production application traffic connect directly to PostgreSQL as a privileged role. Review [Security](SECURITY.md) and [Deployment](DEPLOYMENT.md) before adapting this pattern.


## Before adapting this lab

- Run the example only against a disposable database with synthetic tenant records.
- Ensure application roles are not superusers and do not have `BYPASSRLS` privileges.
- Verify that each tenant has the intended database role mapping and that the token issuer cannot be called by an untrusted client.
- Test a permitted read for tenant A and an attempted read of tenant B's row using the same tenant-A context. The denied case is essential evidence.
- Test writes as well as reads if your application supports them; RLS policies must cover the operations your workload uses.
- Check connection pooling and transaction/session state so one request's tenant context cannot bleed into another request.
- Re-run negative tests after policy, role, driver, proxy, or pool configuration changes.

A tutorial pass is not a substitute for reviewing the real schema and policy set. PostgreSQL superusers and roles with RLS bypass privileges can defeat ordinary row policies. Keep administrative access separate from application traffic and review [Security](SECURITY.md) and [Deployment](DEPLOYMENT.md).
