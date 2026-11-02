# Security Policy

This policy covers the **public Proxima Engine repository**. The hosted Agata Proxima Cloud application is a separate product and is outside this repository's public source scope.

## Report a vulnerability privately

**Do not open a public issue or pull request for an exploitable vulnerability.** Do not publish credentials, signing keys, tenant data, customer information, or sensitive logs.

Use the maintainer contact route published through the [Agata Proxima GitHub organization](https://github.com/Agata-Proxima) to request private disclosure instructions. Include the affected repository, commit SHA or release, impact, safe reproduction steps, and a proposed mitigation when available. Do not include live secrets or customer data.

Please allow maintainers reasonable time to investigate, coordinate a fix, and prepare responsible disclosure before publishing exploit details.

## Scope

In scope: vulnerabilities in the source code and documented behavior of this public Engine repository, including tenant-token validation, PostgreSQL protocol handling, role routing, TLS configuration, local status/metrics surfaces, attestation, and break-glass/replay controls.

Hosted account data, private Cloud source, internal hosted endpoints, credentials, and another customer's information are outside the scope of this public repository.

For hosted account or billing questions, use the official [Agata Proxima website](https://agataproxima.com).

## Safe testing

- Test only systems you own or have explicit permission to assess.
- Use disposable databases and synthetic tenants.
- Obtain written authorization before testing shared or hosted services.
- Do not access, modify, or retain another tenant's data.
- Redact secrets and personal data before sharing diagnostics.

## Information to include

When reporting an Engine issue, include the exact commit SHA, operating system, Rust toolchain, relevant configuration variable names (never secret values), and the smallest safe reproduction.

## Security model

The Engine verifies signed tenant context when enforcement is configured and maps the verified tenant to a PostgreSQL role. PostgreSQL grants and RLS policies enforce row-level access. Operators configure trusted token issuance, TLS, secret storage, network exposure, backups, and deployment verification.

See [the Engine security architecture](docs/SECURITY.md), [Tenant isolation](docs/TENANT_ISOLATION.md), and [Deployment](docs/DEPLOYMENT.md).
