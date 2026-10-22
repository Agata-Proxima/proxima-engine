# Contributing

Thank you for considering a contribution to Agata Proxima Engine.

## Before opening an issue

Search existing issues first. For security vulnerabilities, do not open a public issue; follow [SECURITY.md](SECURITY.md).

## Development workflow

1. Fork the repository and create a focused branch.
2. Keep changes limited to one concern where practical.
3. Add or update tests for behavior changes and bug fixes.
4. Run the checks below.
5. Open a pull request with the motivation, implementation summary, test results, compatibility impact, and any security implications.

## Required local checks

- cargo test -p proxima-engine
- cargo build -p proxima-engine --release

The GitHub Actions workflow runs these commands for pushes to and pull requests against main.

## Review expectations

- Prefer clear, idiomatic Rust and explicit error handling.
- Avoid logging credentials, signing keys, tenant tokens, or sensitive connection data.
- Do not add production secrets, customer information, or private platform code.
- Explain changes to protocol behavior, tenant-token validation, TLS, attestation, or emergency-access controls carefully.
- Update documentation when environment variables, defaults, security assumptions, or public behavior changes.
- Do not claim security certification or guarantees that have not been independently established.

## Scope

This repository contains the public engine component. Changes to the private hosted control plane, commercial platform, customer account surfaces, or billing system do not belong here.
