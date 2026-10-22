# Architecture

## Component boundary

proxima-engine is the public Rust runtime component. The hosted Agata Proxima control plane is a separate, private repository and is not included in this source tree.

At a high level, the engine accepts client connections, parses PostgreSQL protocol messages, applies configured tenant/session controls, and connects to an upstream database. It also contains TLS handling, metrics, an embedded status dashboard, and attestation-related tooling.

## Source map

| Module | Responsibility |
| --- | --- |
| src/main.rs | Engine process startup and runtime wiring |
| src/config.rs | Environment-driven runtime configuration and validation |
| src/protocol.rs | PostgreSQL wire-protocol packet structures and parsing |
| src/tenant.rs | Signed tenant-token parsing, validation, and emergency-token replay checks |
| src/session.rs | Session and connection handling |
| src/policy.rs | Policy-related logic |
| src/tls.rs | TLS transport components |
| src/attestation.rs | Local attestation ledger and verification logic |
| src/metrics.rs | Runtime metrics |
| src/dashboard.rs | Embedded HTTP health/status dashboard |
| src/security_breaker.rs | Security breaker controls |
| src/break_glass.rs | Emergency access mechanisms |
| src/bin/ | Supporting verification and break-glass command-line tools |
| tests/ | Engine integration tests |

## Runtime dependencies

The engine uses Tokio for asynchronous I/O, Rustls for TLS, Reqwest for HTTP client operations, and Tokio Postgres for PostgreSQL connectivity. See [Cargo.toml](../crates/proxima-engine/Cargo.toml) for the authoritative dependency list.

Configuration is loaded from environment variables. The exact defaults and validation rules are in [config.rs](../crates/proxima-engine/src/config.rs); the public summary is in [CONFIGURATION.md](CONFIGURATION.md).

## Trust boundaries

- Client connections are untrusted input. Protocol parsers and connection handlers must validate input and fail safely.
- Tenant identity and authorization depend on correctly configured signing keys and the token-validation path; do not treat a tenant identifier supplied by a client as proof of authorization.
- Upstream database access depends on the deployment's database credentials, network policy, and TLS configuration.
- Attestation evidence is only as trustworthy as its signing-key protection, storage integrity, and the process that creates the evidence.
- The embedded dashboard is an operational surface. Keep it bound to a trusted interface and place suitable access controls around it before exposing it beyond localhost.
- The private control plane is outside this repository. This repository alone does not provide the complete hosted Agata Proxima service.

This document describes the current source layout, not a formal verification of every path. Review the implementation and deployment configuration before relying on it for sensitive workloads.
