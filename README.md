# Agata Proxima Engine

The standalone Rust engine component of Agata Proxima.

## Scope

This repository is intended to publish the engine component only. It does not include the hosted control plane, dashboard, frontend, billing implementation, or hosted-service deployment configuration.

## Build and test

Requires a current stable Rust toolchain.

```sh
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

## Project status

This is the public-engine extraction staging snapshot. Before public release, verify the full test suite and review the licence, dependencies, and release contents.
