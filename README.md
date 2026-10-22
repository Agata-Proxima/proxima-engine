# Proxima Engine

[![Rust CI](https://github.com/Agata-Proxima/proxima-engine/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Agata-Proxima/proxima-engine/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)

**A Rust-based PostgreSQL connectivity engine with tenant-aware session controls, TLS options, operational metrics, and local attestation capabilities.**

Proxima Engine is the public, open-source engine component of Agata Proxima. The hosted Agata Proxima service—including its private control plane, customer dashboard, account management, and billing—is a separate product and is not part of this repository.

## Get running

You need a stable Rust toolchain and a reachable PostgreSQL server.

```bash
git clone https://github.com/Agata-Proxima/proxima-engine.git
cd proxima-engine
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

Configure the PostgreSQL server to proxy to, then start the engine:

```bash
# Change this to the address of your PostgreSQL server.
export PROXIMA_UPSTREAM_ADDR=127.0.0.1:5432
# Keep the listener local for your first run.
export PROXIMA_LISTEN_ADDR=127.0.0.1:6432
cargo run -p proxima-engine
```

In another terminal, connect your PostgreSQL client to the engine's local listener instead of the database's direct port. For example:

```bash
psql "host=127.0.0.1 port=6432 dbname=postgres user=YOUR_DATABASE_USER"
```

Use a database user and database name that exist on your own PostgreSQL server. This is a connection path through the engine, not a bundled database; PostgreSQL must already be running and reachable.

**Documentation hub:** [All developer docs](docs/README.md) · [Quickstart](docs/QUICKSTART.md) · [Network/API reference](docs/API.md) · [Configuration](docs/CONFIGURATION.md)

## What is in this repository?

| Capability | Where to inspect it |
| --- | --- |
| PostgreSQL protocol proxy and session flow | [protocol](crates/proxima-engine/src/protocol.rs) · [session](crates/proxima-engine/src/session.rs) |
| Tenant-token verification and tenant context | [tenant](crates/proxima-engine/src/tenant.rs) · [policy](crates/proxima-engine/src/policy.rs) |
| Client and upstream TLS configuration | [TLS](crates/proxima-engine/src/tls.rs) · [configuration](crates/proxima-engine/src/config.rs) |
| Attestation ledger and evidence handling | [attestation](crates/proxima-engine/src/attestation.rs) |
| Connection/query metrics | [metrics](crates/proxima-engine/src/metrics.rs) |
| Local status dashboard and health route | [dashboard](crates/proxima-engine/src/dashboard.rs) |
| Security breaker and emergency access controls | [security breaker](crates/proxima-engine/src/security_breaker.rs) · [break-glass](crates/proxima-engine/src/break_glass.rs) |

These links point to implementation, not marketing claims. Review the code, tests, and [security notes](docs/SECURITY.md) to understand the behavior and limits of each component.

## Developer documentation

- **[Quickstart](docs/QUICKSTART.md)** — prerequisites, build, configure, start, and verify a local connection.
- **[Network/API reference](docs/API.md)** — PostgreSQL listener, local dashboard routes, metrics, and integration boundaries.
- **[Configuration](docs/CONFIGURATION.md)** — environment variables, defaults, and validation.
- **[Architecture](docs/ARCHITECTURE.md)** — components and trust boundaries.
- **[Security model and limitations](docs/SECURITY.md)** — TLS, tenant verification, deployment precautions, and known boundaries.
- **[Contributing](CONTRIBUTING.md)** — development workflow and pull requests.
- **[Security reporting](SECURITY.md)** — responsible vulnerability disclosure.
- **[Releases](https://github.com/Agata-Proxima/proxima-engine/releases)** · **[CI runs](https://github.com/Agata-Proxima/proxima-engine/actions)** · **[Issues](https://github.com/Agata-Proxima/proxima-engine/issues)**

## Engine repository vs. hosted platform

**Public here:** engine source, engine tests, build workflow, local dashboard assets, configuration and integration documentation.

**Private and separate:** hosted control plane, commercial platform implementation, customer accounts and organizations, billing, and the hosted customer dashboard. This repository does not provide a way to administer the hosted service or reproduce its private dashboard.

To use the hosted Agata Proxima product, visit [agataproxima.com](https://agataproxima.com). Hosted account and dashboard functionality is accessed there; it is not an API promised by this source repository.

## Verify before deployment

```bash
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

GitHub Actions runs the test suite and release build for changes to `main` and pull requests targeting `main`. Check the live [workflow status](https://github.com/Agata-Proxima/proxima-engine/actions) for the commit you are evaluating. A green build is evidence that those automated checks passed; it is not a substitute for an independent security audit or deployment-specific validation.

Start with loopback listeners, configure explicit security settings, and review [Security](docs/SECURITY.md) before exposing the engine to a network. Never commit real credentials, signing keys, customer data, or private infrastructure configuration.

## Contributing

Bug reports and focused pull requests are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) first. Include reproducible steps and tests for behavioral changes, and clearly describe security or compatibility impact.

## License

MIT. See [LICENSE](LICENSE).
