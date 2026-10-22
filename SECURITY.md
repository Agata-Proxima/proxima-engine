# Security Policy

Agata Proxima takes reports about security weaknesses seriously. This policy covers the **public Proxima Engine repository**. It does not publish or describe private hosted-service implementation details.

## Report a vulnerability privately

**Do not open a public issue or pull request for an exploitable vulnerability.** Do not publish credentials, signing keys, tenant data, customer information, or sensitive logs.

Preferred contact: [support@agataproxima.com](mailto:support@agataproxima.com) with the subject **Private security report — Proxima Engine**. Do not include live secrets or customer data in the email.

Include, when safe:
- Affected repository, commit SHA or release and the Engine version.
- Impact and conditions required to reproduce the issue.
- A minimal proof of concept using synthetic data.
- Relevant sanitized logs and any known mitigations.

Please allow maintainers reasonable time to investigate, coordinate a fix, and prepare a responsible disclosure before publishing exploit details.

## Scope

In scope: vulnerabilities in the source code and documented behavior of this public Engine repository, including tenant-token validation, PostgreSQL protocol handling, role routing, TLS configuration, local status/metrics surfaces, attestation, and break-glass/replay controls.

Out of scope for this repository's public issue tracker: requests to expose private hosted-service code, account data, credentials, private endpoints, or another customer's information. For hosted account or billing questions, use the official [website](https://agataproxima.com) and [support contact](mailto:support@agataproxima.com).

## Safe testing

- Test only systems you own or have explicit permission to assess.
- Use disposable databases and synthetic tenants.
- Avoid denial-of-service testing against shared or hosted services without written authorization.
- Do not access, modify, or retain another tenant's data.
- Redact secrets and personal data before sharing diagnostics.

## Supported versions and verification

When reporting a bug, include the exact commit SHA, operating system, Rust toolchain, configuration names (never secret values), and the smallest safe reproduction.

## Security claims

A passing CI run shows that the configured automated checks passed for that commit; it is not a security certification or proof that every deployment is safe. Operators remain responsible for PostgreSQL grants/RLS, trusted token issuance, TLS, secret management, network exposure, backups, and testing denied access paths. See [the Engine security model](docs/SECURITY.md).
