# Support and troubleshooting

## Find the right route

- Setup: [Quickstart](QUICKSTART.md), [Configuration](CONFIGURATION.md), [Operations](OPERATIONS.md).
- Tenant access concerns: stop sensitive traffic and review [Security](SECURITY.md) and [Tenant isolation](TENANT_ISOLATION.md).
- Hosted account, team, node, or billing questions: use [agataproxima.com](https://agataproxima.com).
- Public Engine bugs: file a [GitHub issue](https://github.com/Agata-Proxima/proxima-engine/issues) with a minimal reproduction and commit SHA.
- Potential vulnerability: use the private process in the repository [SECURITY.md](../SECURITY.md), not a public issue.

## Contact

Intended public contact: [support@agataproxima.com](mailto:support@agataproxima.com). The mailbox must be configured and monitored before it is relied upon. If email fails, use the official website.

## Include in a support request

- Engine version or exact commit SHA.
- Operating system, container runtime, and Rust version if relevant.
- Exact command/request, sanitized response, and expected behavior.
- Configuration variable names, never their secret values.
- A minimal reproduction using synthetic data.

Never send passwords, signing keys, raw access tokens, payment data, customer records, private keys, or unredacted tenant logs.

## First checks

1. Check [health and status routes](API.md).
2. Confirm the upstream PostgreSQL host, port, database, and credentials.
3. Check DNS and connectivity from the Engine runtime.
4. Compare environment variable names with [Configuration](CONFIGURATION.md).
5. For tenant issues, test allowed and denied access paths using [Tenant isolation](TENANT_ISOLATION.md).
6. If security is implicated, stop public disclosure and use the private vulnerability route.

This guide does not imply a response-time or uptime SLA.
