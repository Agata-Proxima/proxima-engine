# Developer evaluation challenge

Agata Proxima is an open-source Rust PostgreSQL wire-protocol proxy. It does not replace PostgreSQL, application authorization, grants, or RLS.

## Reproduce the boundary

Use a disposable database with synthetic records. Start with the [Quickstart](QUICKSTART.md), then run the [tenant-isolation lab](TENANT_ISOLATION.md). The default quickstart does not enable tenant enforcement and does not prove isolation.

Verify: (1) Tenant A can read its own synthetic row; (2) Tenant A cannot see Tenant B's row; (3) cross-tenant writes are rejected by PostgreSQL RLS; (4) invalid or missing signed tenant context is rejected when enforcement is enabled. Record the Engine commit, PostgreSQL version, non-secret commands, and sanitized output. Never publish signing keys, passwords, tokens, or customer data. These checks do not certify production security.

## Participate

[Star/watch the Engine](https://github.com/Agata-Proxima/proxima-engine) to follow changes. [Open an issue](https://github.com/Agata-Proxima/proxima-engine/issues) for reproducible bugs, driver compatibility, confusing setup, or improvement requests. Read [Contributing](../CONTRIBUTING.md) before a pull request. Report vulnerabilities privately through [SECURITY.md](../SECURITY.md), not in public issues.

## Launch assets checklist

- Square logo with clear contrast.
- Homepage screenshot with product promise and developer CTA.
- Public GitHub README screenshot showing the quickstart.
- Terminal screenshot of successful tenant read and denied cross-tenant operation, using synthetic data only.
- Optional short recording: start Engine, run permitted and denied test, show documentation. Exclude all secrets.

Website: https://agataproxima.com

Engine: https://github.com/Agata-Proxima/proxima-engine
