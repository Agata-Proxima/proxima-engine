# Agata Proxima Engine

**An open-source Rust security-infrastructure engine** from Agata Proxima.

This repository publishes the engine code for developers to inspect, build, test, modify, and contribute to. The hosted control plane, customer dashboard, account management, and billing implementation are kept in the separate private platform repository.

[Browse the source code](https://github.com/Agata-Proxima/proxima-engine/tree/main/crates/proxima-engine/src) · [Run CI](https://github.com/Agata-Proxima/proxima-engine/actions) · [MIT licence](LICENSE)

## What is included

The engine source includes:

- **Tenant and session enforcement** — [tenant.rs](crates/proxima-engine/src/tenant.rs), [session.rs](crates/proxima-engine/src/session.rs)
- **Attestation support** — [attestation.rs](crates/proxima-engine/src/attestation.rs) and the `proxima-attest-verify` command
- **Policy and security controls** — [policy.rs](crates/proxima-engine/src/policy.rs), [security_breaker.rs](crates/proxima-engine/src/security_breaker.rs), [break_glass.rs](crates/proxima-engine/src/break_glass.rs)
- **Protocol and TLS** — [protocol.rs](crates/proxima-engine/src/protocol.rs), [tls.rs](crates/proxima-engine/src/tls.rs)
- **Metrics and health dashboard** — [metrics.rs](crates/proxima-engine/src/metrics.rs), [dashboard.rs](crates/proxima-engine/src/dashboard.rs)
- **Configuration and runtime** — [config.rs](crates/proxima-engine/src/config.rs), [main.rs](crates/proxima-engine/src/main.rs)
- **Integration tests** — [postgres_integration.rs](crates/proxima-engine/tests/postgres_integration.rs)

## Quick start

Requirements: a current stable Rust toolchain.

Clone the repository:

```sh
git clone https://github.com/Agata-Proxima/proxima-engine.git
cd proxima-engine
```

Run the tests and build an optimized binary:

```sh
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

The executable is produced under `target/release/`. Runtime configuration is read from environment variables; review [config.rs](crates/proxima-engine/src/config.rs) before running it in an environment.

## Repository layout

```text
.
├── Cargo.toml
├── crates/
│   └── proxima-engine/
│       ├── Cargo.toml
│       ├── src/       # Rust engine modules and CLI tools
│       └── tests/     # Integration tests
├── dashboard/         # HTML embedded into the engine
├── frontend/src/assets/ # Logo asset embedded into the dashboard
├── .github/workflows/ # Automated Rust CI
└── LICENSE
```

## What is not included

This public repository does **not** contain the hosted control plane, customer account and organization management, billing/payment integrations, or the complete hosted frontend. Those remain separate from the open-source engine.

## Contributing

1. Fork the repository and create a branch.
2. Make a focused change with tests where appropriate.
3. Run `cargo test -p proxima-engine` and `cargo build -p proxima-engine --release`.
4. Open a pull request describing the change and test results.

## Licence

MIT. See [LICENSE](LICENSE).
