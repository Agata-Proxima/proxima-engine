# Security model and limitations

Agata Proxima Engine is security-sensitive infrastructure. This document describes relevant configuration and trust assumptions; it is not a security certification.

## Report a vulnerability

Please do not disclose exploitable vulnerabilities, credentials, customer data, or sensitive logs in public GitHub issues. Until a dedicated private security-reporting channel is published, contact the maintainers through the contact options on the [Agata Proxima GitHub organization](https://github.com/Agata-Proxima) and request a private disclosure route. Include the affected commit, impact, reproduction steps, and any proposed mitigation. Avoid attaching real secrets or production data.

## Deployment checklist

Before connecting the engine to sensitive data:

- Pin and review the exact source commit you build.
- Keep signing keys, TLS private keys, database credentials, and other secrets out of Git and logs.
- Configure a strong tenant signing key. The engine configuration rejects a configured tenant signing key shorter than 32 bytes; an absent key means tenant-token enforcement may not be configured.
- Configure TLS certificates and keys together when enabling client TLS.
- When enabling upstream TLS, configure a trusted CA file. For verify-full, configure the upstream server name as well.
- Bind client and dashboard listeners to intended interfaces only. The default listener addresses use loopback, but deployment overrides can expose them.
- Restrict access to the dashboard and metrics endpoints at the network layer as appropriate.
- Use least-privilege database credentials and restrict network reachability to trusted clients and upstreams.
- Protect the attestation signing key and ledger storage against unauthorized modification.
- Run the automated test suite and validate the actual deployment topology before production use.

## Configuration details

The implementation is the source of truth:
- [Runtime configuration](../crates/proxima-engine/src/config.rs)
- [Tenant-token handling](../crates/proxima-engine/src/tenant.rs)
- [TLS implementation](../crates/proxima-engine/src/tls.rs)
- [Attestation ledger](../crates/proxima-engine/src/attestation.rs)

## Scope and guarantees

A green CI badge means the repository's configured automated tests and release build passed for the commit tested. It does not establish that the system is free of vulnerabilities, independently audited, penetration-tested, compliant with a particular standard, or suitable for every production environment.

Security controls can be weakened by incorrect keys, exposed listeners, unsafe network policy, misconfigured TLS, compromised hosts, or integrations outside this repository. Operators remain responsible for validating their deployment.
