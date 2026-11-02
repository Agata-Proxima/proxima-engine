# Support and issue reporting

Use the route that matches the request.

## Technical documentation

- [Quickstart](QUICKSTART.md) — start the Engine and query PostgreSQL.
- [Configuration](CONFIGURATION.md) — environment variables and defaults.
- [Operations](OPERATIONS.md) — logs, health checks, and troubleshooting.
- [Tenant isolation](TENANT_ISOLATION.md) — tenant roles, signed context, and RLS verification.

## Public Engine issues

Use [GitHub Issues](https://github.com/Agata-Proxima/proxima-engine/issues) for reproducible Engine bugs, driver integration problems, documentation corrections, and feature requests.

Include:

- Engine version or exact commit SHA.
- Operating system, container runtime, and Rust version where relevant.
- Exact command or request, sanitized response, and expected behavior.
- Configuration variable names, without secret values.
- A minimal reproduction using synthetic data.

## Hosted account and billing

For account, organization, node-management, plan, and billing workflows, use the official [Agata Proxima website](https://agataproxima.com). The hosted product is separate from the public Engine issue tracker.

## Security reports

Do not publish exploitable vulnerabilities, credentials, tenant data, or sensitive logs in public issues. Follow the repository [Security Policy](../SECURITY.md) for private vulnerability reporting.

## Information to keep private

Never include passwords, signing keys, access tokens, payment data, customer records, private keys, or unredacted tenant logs in a support request or issue.

## First checks

1. Check the [health and status routes](API.md).
2. Confirm the upstream PostgreSQL host, port, database, and credentials.
3. Check DNS and connectivity from the Engine runtime.
4. Compare environment variable names with [Configuration](CONFIGURATION.md).
5. For tenant issues, run allowed and denied access tests using [Tenant isolation](TENANT_ISOLATION.md).
6. For suspected vulnerabilities, stop public discussion and follow the private reporting process.
