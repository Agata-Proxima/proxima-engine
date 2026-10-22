# Verification and test guide

Use these commands to validate the Engine source you have checked out. Record the commit SHA and toolchain with your results.

## 1. Run unit and integration tests

```bash
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

The PostgreSQL-backed integration test connects to a local PostgreSQL service at `127.0.0.1:5432` using:
- Database: `proxima_dev`
- User: `proxima`
- Password: `proxima-dev-only`

For a local run of the integration test, start a disposable PostgreSQL 17 instance with those credentials. Example:

```bash
docker run --rm --name proxima-ci-postgres \
  -e POSTGRES_USER=proxima \
  -e POSTGRES_PASSWORD=proxima-dev-only \
  -e POSTGRES_DB=proxima_dev \
  -p 127.0.0.1:5432:5432 postgres:17
```

Keep it running in one terminal, then run `cargo test -p proxima-engine` in another. The test creates tenant roles and an RLS table in that disposable database. Do not point this test at a database containing data you need; it drops the `proxima_e2e` schema and changes the test roles.

The GitHub Actions workflow provisions this disposable PostgreSQL service automatically.

## 2. Test the running HTTP surfaces

With the [quickstart](QUICKSTART.md) running:

```bash
curl -fsS http://127.0.0.1:9090/healthz
curl -fsS http://127.0.0.1:9080/api/status
curl -fsS http://127.0.0.1:9090/metrics
```

A health response alone is not a database test. Run a query through port 6432 as shown in the quickstart.

## 3. Verify an attestation ledger

The verifier checks the local hash-linked ledger and validates its signatures. Point it at the ledger you want to inspect and supply the same signing key, organization ID, and node ID used when the records were created. The CLI does not automatically read the Engine identity JSON file.

```bash
export PROXIMA_ATTESTATION_LEDGER_PATH=data/proxima-attestations.jsonl
export PROXIMA_ORGANIZATION_ID='<recorded-organization-id>'
export PROXIMA_NODE_ID='<recorded-node-id>'
# Prefer the node identity key (hex) or the configured attestation signing key.
export PROXIMA_NODE_SIGNING_KEY_HEX='<64-hex-character-node-key>'
cargo run -p proxima-engine --bin proxima-attest-verify
```

- Exit code 0: one or more records were present and the chain verified.
- Exit code 2: no records exist or the chain is invalid.
- Exit code 1: the ledger could not be read or verified because of an I/O/runtime error.

An empty ledger is not proof that attestation succeeded. Configure the signing credential and the required attestation settings before expecting records. See [Configuration](CONFIGURATION.md).

## 4. Generate an emergency token

The command-line tool issues a short-lived, signed break-glass token for a tenant. It requires the same tenant signing secret configured in the Engine:

```bash
export PROXIMA_TENANT_SIGNING_KEY='<local-test-secret-at-least-32-bytes>'
cargo run -p proxima-engine --bin proxima-break-glass-token -- tenant_a 300
```

The lifetime is limited to 900 seconds (15 minutes). The Engine records used nonces in its replay directory and rejects token reuse. Keep the token private; this command prints a credential to stdout. Do not generate emergency tokens for routine access or store them in logs.

## 5. CI checks

The repository workflow runs the PostgreSQL-backed test suite and release build on pushes to `main` and pull requests targeting `main`. Review the exact [GitHub Actions run](https://github.com/Agata-Proxima/proxima-engine/actions) for your commit.

CI success does not replace TLS verification, production-like integration tests, independent security review, backup/restore testing, or deployment acceptance.


## Evidence checklist

For a useful verification record, capture:

- The exact Git commit SHA and, for containers, image digest.
- Operating system, CPU architecture, Rust toolchain, and container runtime versions.
- Which tests ran and their actual exit status.
- Whether PostgreSQL-backed tests used a disposable database and completed successfully.
- The results of a positive tenant access test and a negative cross-tenant access test.
- TLS certificate validation and connection behavior for the intended deployment.
- The attestation verifier command, trusted public-key fingerprint, expected organization/node context, and result, without recording private keys or tokens.

A green CI badge means only that the configured workflow passed for the referenced commit. It does not mean a customer's runtime configuration, database grants, RLS policies, network exposure, or incident procedures have been verified. See [Security](SECURITY.md) and [Deployment](DEPLOYMENT.md).
