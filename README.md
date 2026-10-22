# Agata Proxima Engine

[![Rust CI](https://github.com/Agata-Proxima/proxima-engine/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Agata-Proxima/proxima-engine/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

**A Rust security-infrastructure engine for tenant-aware database connectivity, protocol handling, transport security, and verifiable operational evidence.**

Agata Proxima is split into two deliberately separate parts: this repository publishes the **engine** for public inspection and community contribution; the hosted control plane and commercial platform remain private.

**Start here:** [Architecture](docs/ARCHITECTURE.md) · [Security model and limitations](docs/SECURITY.md) · [Configuration reference](docs/CONFIGURATION.md) · [Contributing](CONTRIBUTING.md) · [Releases](https://github.com/Agata-Proxima/proxima-engine/releases) · [CI results](https://github.com/Agata-Proxima/proxima-engine/actions)

## Why this repository exists

Infrastructure software should be inspectable. This repository exposes the Rust engine implementation, its protocol and security-related modules, test suite, build workflow, and contributor process without publishing the hosted product's control plane, customer-management surfaces, or billing implementation.

The public source is intended for engineers evaluating the implementation, reviewing changes, running tests, and proposing focused improvements. Please read the security and deployment notes before using it with real workloads.

## Engine capabilities

| Area | Implementation |
| --- | --- |
| PostgreSQL wire-protocol handling | Startup packets, protocol messages, frame validation, and protocol constants in [protocol.rs](crates/proxima-engine/src/protocol.rs) |
| Tenant context | Signed tenant-token parsing and validation, expiry checks, and emergency-token replay controls in [tenant.rs](crates/proxima-engine/src/tenant.rs) |
| Session and policy controls | Session handling and policy modules in [session.rs](crates/proxima-engine/src/session.rs) and [policy.rs](crates/proxima-engine/src/policy.rs) |
| Transport security | TLS configuration and client/upstream TLS components in [tls.rs](crates/proxima-engine/src/tls.rs) and [config.rs](crates/proxima-engine/src/config.rs) |
| Attestation evidence | Hash/HMAC-backed attestation ledger and verification tooling in [attestation.rs](crates/proxima-engine/src/attestation.rs) |
| Operational visibility | Metrics and an embedded health/status dashboard in [metrics.rs](crates/proxima-engine/src/metrics.rs) and [dashboard.rs](crates/proxima-engine/src/dashboard.rs) |
| Safety controls | Security breaker and break-glass mechanisms in [security_breaker.rs](crates/proxima-engine/src/security_breaker.rs) and [break_glass.rs](crates/proxima-engine/src/break_glass.rs) |

This table identifies source areas; it is not a claim of independent security certification or a guarantee that every deployment is secure by default.

## Repository boundaries

**Included here**
- Rust engine source and command-line utilities
- Engine-specific tests
- Embedded dashboard assets required to build the engine
- Rust CI workflow and developer documentation

**Not included here**
- Hosted control plane and its APIs
- Customer dashboard and account/organization management
- Billing, payment-provider integrations, and commercial service configuration
- Production credentials, private infrastructure configuration, or customer data

The private platform is not required to browse or build the public source. Engine features that depend on a compatible control-plane service or external database still require that dependency to be configured.

## Build from source

### Requirements

- A current stable Rust toolchain (rustc and cargo)
- A supported operating system and network access to download Rust dependencies
- PostgreSQL only when running tests or scenarios that explicitly require a database

### Clone, test, and build

```bash
git clone https://github.com/Agata-Proxima/proxima-engine.git
cd proxima-engine

# Run the engine test suite
cargo test -p proxima-engine

# Build the optimized executable
cargo build -p proxima-engine --release
```

The release binary is written under `target/release/`. The CI workflow runs the same test and release-build commands on pushes and pull requests to `main`.

### Configuration

The runtime reads configuration from environment variables. Defaults and validation are implemented in [config.rs](crates/proxima-engine/src/config.rs); attestation settings are defined in [attestation.rs](crates/proxima-engine/src/attestation.rs).

Before running the engine, read the [configuration reference](docs/CONFIGURATION.md), set explicit addresses and security settings for your environment, and do not expose listeners publicly until you have reviewed the trust boundaries and access controls. Do not put production secrets in source control, issues, or pull requests.

## Project maturity

This repository is an active public source release. A passing CI run means the configured automated test and build commands passed for that commit; it does **not** mean the engine has undergone an independent security audit, has been certified for a regulated workload, or is suitable for every production environment. Pin the exact commit you evaluate and validate the integration in your own environment before relying on it.

## Security reporting

Please do not publish exploitable vulnerabilities, credentials, tenant data, or sensitive logs in public issues. Follow [SECURITY.md](SECURITY.md) for responsible reporting guidance.

## Contributing

Issues and focused pull requests are welcome. Start with [CONTRIBUTING.md](CONTRIBUTING.md), include tests for behavioral changes, and explain any security or compatibility impact.

## License

MIT. See [LICENSE](LICENSE).
