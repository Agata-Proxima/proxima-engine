# Security model and limitations

Proxima Engine is security-sensitive infrastructure. This guide describes the controls implemented in this repository and the deployment assumptions they rely on. It is not a security certification.

## Security properties to configure and verify

- **Tenant context:** when `PROXIMA_TENANT_SIGNING_KEY` is set, a valid, unexpired signed tenant token is required during PostgreSQL startup. The Engine rejects missing, malformed, expired, or incorrectly signed tokens.
- **Role routing:** a verified tenant ID is mapped to a PostgreSQL role using `PROXIMA_TENANT_ROLE_PREFIX`. PostgreSQL permissions and RLS policies remain the database-side enforcement mechanism.
- **TLS:** client TLS requires a configured certificate/key pair. Set `PROXIMA_TLS_REQUIRE_CLIENT=true` to reject plaintext clients. For the upstream hop, use `PROXIMA_UPSTREAM_TLS_MODE=verify-full` with a trusted CA and the expected server name.
- **Resource bounds:** connection count, upstream connection timeout, and TLS handshake timeouts are configurable and validated.
- **Violation breaker:** repeated protocol or tenant-security failures can block a peer for a configured cooldown. See the breaker variables in [Configuration](CONFIGURATION.md).
- **Emergency tokens:** break-glass tokens are signed, short-lived (at most 15 minutes), and protected against replay by a local nonce store.
- **Attestation:** records form a hash-linked ledger and are signed when a valid signing credential is configured. Protect the ledger and signing key from modification.

## Before connecting sensitive workloads

- [ ] Pin and review the exact source commit or image digest.
- [ ] Configure tenant-token enforcement; do not assume it is enabled when the signing key is absent.
- [ ] Generate tenant tokens only in a trusted server after verifying the user's tenant authorization.
- [ ] Use non-superuser PostgreSQL roles without `BYPASSRLS`; test both permitted and denied reads/writes.
- [ ] Remove direct privileged database paths that bypass the intended Engine boundary.
- [ ] Configure client TLS and verified upstream TLS; test wrong and expired certificates.
- [ ] Bind the PostgreSQL listener to intended interfaces and restrict ingress with firewall/network policy.
- [ ] Keep the embedded status page and metrics listener on loopback or a private, access-controlled network. The status page has no built-in authentication.
- [ ] Keep node identity files, signing keys, database passwords, and enrollment tokens out of Git and logs.
- [ ] Persist and protect the node identity, replay store, and attestation ledger.
- [ ] Test restarts, upstream outages, token expiry, connection exhaustion, secret rotation, and recovery.
- [ ] Review [Verification](VERIFICATION.md) and [Deployment](DEPLOYMENT.md).

## Important limitations

- If `PROXIMA_TENANT_SIGNING_KEY` is absent, the Engine does not require tenant tokens. A connectivity-only run is not tenant isolation.
- PostgreSQL superusers and roles with `BYPASSRLS` can bypass RLS. Database grants and network topology are part of the security design.
- The embedded status and metrics listeners do not authenticate callers. They should not be publicly exposed.
- The attestation latency value is supplied through configuration; it should not be described as an independently measured benchmark unless you measured and validated it separately.
- A local attestation chain does not protect against an attacker who can alter both the ledger and its keys or host.
- A passing CI run shows only that configured automated checks passed. It does not prove the absence of vulnerabilities, independent audit, compliance, or production readiness.

## Report a vulnerability

Do not publish exploitable vulnerabilities, credentials, tenant data, or sensitive logs in public issues. Use the contact options on the [Agata Proxima GitHub organization](https://github.com/Agata-Proxima) to request a private disclosure route until a dedicated private channel is published. Include the affected commit, impact, reproduction steps, and a proposed mitigation if available. Do not include real secrets or production data.
