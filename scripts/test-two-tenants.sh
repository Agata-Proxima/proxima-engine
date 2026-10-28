#!/usr/bin/env bash
# Disposable PostgreSQL 17 two-tenant acceptance test; never run on production.
set -euo pipefail
export PGHOST=127.0.0.1 PGPORT=5432 PGUSER=proxima PGDATABASE=proxima_dev PGPASSWORD=proxima-dev-only
psql -v ON_ERROR_STOP=1 <<'SQL'
DROP SCHEMA IF EXISTS proxima_cli_e2e CASCADE;
DROP ROLE IF EXISTS proxima_tenant_cli_a;
DROP ROLE IF EXISTS proxima_tenant_cli_b;
CREATE ROLE proxima_tenant_cli_a LOGIN PASSWORD 'tenant-a-password' NOSUPERUSER NOBYPASSRLS;
CREATE ROLE proxima_tenant_cli_b LOGIN PASSWORD 'tenant-b-password' NOSUPERUSER NOBYPASSRLS;
CREATE SCHEMA proxima_cli_e2e;
CREATE TABLE proxima_cli_e2e.records(tenant_id text NOT NULL, secret text NOT NULL);
INSERT INTO proxima_cli_e2e.records VALUES ('cli_a','A-secret'),('cli_b','B-secret');
ALTER TABLE proxima_cli_e2e.records ENABLE ROW LEVEL SECURITY;
ALTER TABLE proxima_cli_e2e.records FORCE ROW LEVEL SECURITY;
CREATE POLICY isolate_a ON proxima_cli_e2e.records TO proxima_tenant_cli_a USING (tenant_id='cli_a') WITH CHECK (tenant_id='cli_a');
CREATE POLICY isolate_b ON proxima_cli_e2e.records TO proxima_tenant_cli_b USING (tenant_id='cli_b') WITH CHECK (tenant_id='cli_b');
GRANT USAGE ON SCHEMA proxima_cli_e2e TO proxima_tenant_cli_a,proxima_tenant_cli_b;
GRANT SELECT,INSERT ON proxima_cli_e2e.records TO proxima_tenant_cli_a,proxima_tenant_cli_b;
SQL
python3 scripts/agata.py init
export PROXIMA_TENANT_SIGNING_KEY
PROXIMA_TENANT_SIGNING_KEY="$(python3 -c 'import json; print(json.load(open(".proxima-dev/dev-config.json"))["tenant_signing_key"])')"
export PROXIMA_LISTEN_ADDR=127.0.0.1:6432 PROXIMA_UPSTREAM_ADDR=127.0.0.1:5432
export PROXIMA_DASHBOARD_ENABLED=false PROXIMA_METRICS_LISTEN_ADDR=127.0.0.1:19090
target/release/proxima-engine > /tmp/proxima-e2e.log 2>&1 &
engine_pid=$!
trap 'kill "$engine_pid" 2>/dev/null || true' EXIT
for i in $(seq 1 40); do
  if curl -fsS http://127.0.0.1:19090/healthz >/dev/null 2>&1; then break; fi
  if ! kill -0 "$engine_pid" 2>/dev/null; then cat /tmp/proxima-e2e.log; exit 1; fi
  sleep 1
done
curl -fsS http://127.0.0.1:19090/healthz >/dev/null
token_a="$(python3 scripts/agata.py token --tenant cli_a --ttl 300)"
token_b="$(python3 scripts/agata.py token --tenant cli_b --ttl 300)"
run_tenant() {
  local token="$1" password="$2" sql="$3"
  PGPASSWORD="$password" PGOPTIONS="-c proxima_tenant_token=$token" psql -X -A -t -v ON_ERROR_STOP=1 \
    "host=127.0.0.1 port=6432 dbname=proxima_dev user=proxima sslmode=disable" -c "$sql"
}
test "$(run_tenant "$token_a" tenant-a-password 'SELECT secret FROM proxima_cli_e2e.records')" = "A-secret"
test "$(run_tenant "$token_b" tenant-b-password 'SELECT secret FROM proxima_cli_e2e.records')" = "B-secret"
test "$(run_tenant "$token_a" tenant-a-password "SELECT count(*) FROM proxima_cli_e2e.records WHERE tenant_id='cli_b'")" = "0"
if run_tenant "$token_a" tenant-a-password "INSERT INTO proxima_cli_e2e.records VALUES ('cli_b','forbidden')" >/dev/null 2>&1; then
  echo "FAIL: cross-tenant insert unexpectedly succeeded" >&2; exit 1
fi
if run_tenant "invalid-token" tenant-a-password 'SELECT 1' >/dev/null 2>&1; then
  echo "FAIL: invalid tenant token unexpectedly accepted" >&2; exit 1
fi
echo "PASS: tenant A/B isolation, cross-tenant read/write denial, invalid token rejection"
