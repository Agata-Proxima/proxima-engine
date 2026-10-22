# Proxima Engine documentation

Use this index to find the right guide without reading the source tree first.

## Start here

- [Quickstart](QUICKSTART.md) — build, run, connect a PostgreSQL client, and verify the local engine.
- [Network/API reference](API.md) — supported PostgreSQL, dashboard, health, and metrics surfaces.
- [Configuration reference](CONFIGURATION.md) — environment variables, defaults, and validation rules.

## Understand the engine

- [Architecture](ARCHITECTURE.md) — components and trust boundaries.
- [Security model and limitations](SECURITY.md) — security controls and deployment considerations.
- [Engine source](../crates/proxima-engine/src/main.rs) — executable entry point.

## Work with the project

- [Contributing guide](../CONTRIBUTING.md) — development and pull-request expectations.
- [Security reporting](../SECURITY.md) — responsible vulnerability disclosure.
- [License](../LICENSE) — MIT license.
- [CI workflows](../.github/workflows/ci.yml) — automated test and release-build commands.
- [GitHub Actions](https://github.com/Agata-Proxima/proxima-engine/actions) — live results for the current branches and commits.
- [Issues](https://github.com/Agata-Proxima/proxima-engine/issues) — report reproducible engine bugs and request improvements.

## Scope boundary

This documentation covers the public engine repository only. The hosted Agata Proxima control plane, customer dashboard, account and organization management, and billing remain part of the separate private service. Hosted-product functionality is not an engine API contract.
