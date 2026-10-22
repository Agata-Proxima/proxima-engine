# Agata Proxima Engine

The standalone Rust engine component of Agata Proxima.

## Scope

This repository contains the engine component only. The hosted control plane, dashboard, frontend, billing implementation, and hosted-service deployment configuration remain separate and are not included here.

## Build and test

Requires a current stable Rust toolchain.

```sh
cargo test -p proxima-engine
cargo build -p proxima-engine --release
```

## Repository layout

- `crates/proxima-engine` — engine source, command-line tools, and integration tests.

## License

MIT. See [LICENSE](LICENSE).
