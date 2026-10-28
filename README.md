<p align="center">
  <img src="frontend/src/assets/agata-proxima-orbit.svg" alt="Agata Proxima orbit logo" width="132">
</p>

<h1 align="center">Agata Proxima</h1>
<p align="center"><strong>Security infrastructure for multi-tenant PostgreSQL applications.</strong></p>
<p align="center">The hosted control plane coordinates your organization and registered nodes. Proxima Engine is the open-source data plane that sits between your application and PostgreSQL.</p>

<p align="center">
  <a href="https://github.com/Agata-Proxima/proxima-engine/actions/workflows/ci.yml"><img alt="CI status" src="https://github.com/Agata-Proxima/proxima-engine/actions/workflows/ci.yml/badge.svg?branch=main"></a>
  <a href="LICENSE"><img alt="MIT license for covered source" src="https://img.shields.io/badge/Engine%20source-MIT-16a34a.svg"></a>
  <a href="https://agataproxima.com"><img alt="Website" src="https://img.shields.io/badge/website-agataproxima.com-16a34a.svg"></a>
  <img alt="Rust" src="https://img.shields.io/badge/Rust-stable-orange.svg">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-wire%20protocol-336791.svg">
</p>

**Start here:** [Quickstart](docs/QUICKSTART.md) · [Local developer mode — no hosted account required](docs/LOCAL_DEVELOPMENT.md) · [Engine CLI](docs/CLI.md) · [Architecture](docs/ARCHITECTURE.md) · [Tenant isolation lab](docs/TENANT_ISOLATION.md) · [API & health](docs/API.md) · [Security](docs/SECURITY.md) · [Hosted platform](https://agataproxima.com)

## What is Agata Proxima?

Agata Proxima is designed to make tenant boundaries easier to operate and verify in applications that use PostgreSQL. It has two deliberately separate parts:

- **Proxima Engine — open source:** a Rust PostgreSQL-protocol proxy. It validates signed tenant context when enabled, maps verified tenant IDs to database roles, relays database sessions, and exposes local health/status/Prometheus metrics.
- **Agata Proxima Cloud — hosted, separate product:** the private management experience for authorized organizations and registered Engine nodes. Account, organization, team, plan, billing, and hosted management workflows are not implemented by this repository and are not licensed by the Engine repository's MIT license.

The Engine is not a replacement for PostgreSQL, your application authorization, or database Row-Level Security (RLS). PostgreSQL grants and RLS policies remain essential. Read the [security model](docs/SECURITY.md) before using sensitive workloads.

## See the request path

![Agata Proxima tenant-isolation architecture](docs/assets/proxima-architecture.svg)\n\n[View full-size vector diagram](docs/assets/proxima-architecture.svg) · [Detailed Mermaid sequence and deployment diagrams](docs/ARCHITECTURE.md)

The dotted line represents the control-plane boundary. This public repository documents the Engine-side contract; it does not expose private Cloud source code or promise that every hosted feature is available in a self-managed installation.

## Run it locally

Requirements: Git and Docker with Compose.

```bash
git clone https://github.com/Agata-Proxima/proxima-engine.git
cd proxima-engine
docker compose -f compose.dev.yaml up --build -d
```

Check the local health and status routes:

```bash
curl -fsS http://127.0.0.1:9090/healthz
curl -fsS http://127.0.0.1:9080/api/status
```

Windows PowerShell equivalent:

```powershell
Invoke-RestMethod http://127.0.0.1:9090/healthz
Invoke-RestMethod http://127.0.0.1:9080/api/status
```

Then run the [full Quickstart](docs/QUICKSTART.md), which performs a real SQL query through the proxy. The Compose profile is a disposable local demo; its example credentials are not for production. The initial connectivity demo does **not** by itself prove tenant isolation.

## A practical path from demo to integration

<details>
<summary><strong>1 — Start the Engine</strong></summary>

Follow [Quickstart](docs/QUICKSTART.md). It starts the Engine and PostgreSQL locally, checks health, inspects status, and sends a query through the proxy.
</details>

<details>
<summary><strong>2 — Understand the trust boundary</strong></summary>

Read [Architecture](docs/ARCHITECTURE.md) and [Security](docs/SECURITY.md). Decide which process is trusted to authorize a user for a tenant and mint a short-lived signed tenant token. Never put the signing key in a browser or mobile application.
</details>

<details>
<summary><strong>3 — Prove tenant separation</strong></summary>

Run [Tenant isolation tutorial](docs/TENANT_ISOLATION.md). It creates two non-superuser roles, applies PostgreSQL RLS, and demonstrates both allowed and denied access. Do not skip negative tests.
</details>

<details>
<summary><strong>4 — Connect your application</strong></summary>

Keep your existing PostgreSQL driver and change its endpoint to the Engine. Copy a server-side example from [Node.js, Python, or Rust integration recipes](docs/INTEGRATIONS.md). Production connections must use correctly verified TLS.
</details>

<details>
<summary><strong>5 — Operate and verify</strong></summary>

Use [API, health, and metrics](docs/API.md), [Operations](docs/OPERATIONS.md), and [Verification & CLI](docs/VERIFICATION.md). A healthy HTTP endpoint is not a substitute for a database query, RLS tests, or review of your production configuration.
</details>

## Documentation

| Guide | What it answers |
|---|---|
| [Quickstart](docs/QUICKSTART.md) | Run a local stack and execute a real SQL query |
| [Engine CLI](docs/CLI.md) | Local init, token minting, token verification, and health checks |
| [Local developer mode](docs/LOCAL_DEVELOPMENT.md) | Generate local node identity and short-lived test tenant tokens without Cloud or billing |
| [Architecture](docs/ARCHITECTURE.md) | Components, request lifecycle, trust boundaries |
| [Configuration](docs/CONFIGURATION.md) | Environment variables, defaults, and security-sensitive settings |
| [Tenant isolation](docs/TENANT_ISOLATION.md) | Signed tenant context, PostgreSQL roles, RLS, negative tests |
| [Integrations](docs/INTEGRATIONS.md) | psql, Node.js, Python, and Rust driver examples |
| [API reference](docs/API.md) | Implemented HTTP routes, PostgreSQL endpoint, metrics |
| [Node enrollment](docs/NODE_ENROLLMENT.md) | Engine-side enrollment and persistent node identity |
| [Deployment](docs/DEPLOYMENT.md) | Container deployment and production hardening |
| [Operations](docs/OPERATIONS.md) | Troubleshooting, health checks, logs, recovery |
| [Verification & CLI](docs/VERIFICATION.md) | Tests, attestation verification, break-glass tooling |
| [Security model](docs/SECURITY.md) | Implemented controls, limitations, deployment checklist |
| [Cloud control plane](docs/CLOUD_CONTROL_PLANE.md) | What stays hosted/private and how the parts relate |
| [Plans and billing](docs/BILLING.md) | Hosted plans, usage concepts, upgrade path, billing boundaries |
| [Support](docs/SUPPORT.md) | Where to ask questions and report security issues |
| [Contributing](CONTRIBUTING.md) | Local development and pull-request expectations |
| [Code of Conduct](CODE_OF_CONDUCT.md) | Community standards |
| [Copyright & brand](COPYRIGHT.md) | Source license versus protected brand assets |
| [Open-source scope and responsibility](docs/OPEN_SOURCE_SCOPE.md) | Self-managed use, warranty disclaimer, responsibility, and support boundaries |
| [Security policy](SECURITY.md) | Private vulnerability reporting guidance |

## Hosted Cloud, plans, and support

- **Website and hosted account:** [agataproxima.com](https://agataproxima.com)
- **Plans and billing:** review plans, usage limits, invoices, and upgrades through the hosted account workflow. See [Plans and billing](docs/BILLING.md).
- **Support:** [support@agataproxima.com](mailto:support@agataproxima.com)
- **Security vulnerabilities:** report them privately as described in [SECURITY.md](SECURITY.md).

```mermaid
flowchart TD
  Dev[Developer] --> Docs[Public Engine documentation]
  Docs --> Engine[Open-source Proxima Engine]
  Customer[Customer organization] --> Cloud[Agata Proxima hosted control plane]
  Cloud -->|authorized node enrollment| Engine
  App[Application / PostgreSQL driver] --> Engine
  Engine --> DB[(PostgreSQL)]
  Cloud -.->|"Account, team, plans and billing"| Customer
```

The hosted account, organization, team, node-registration, and billing workflows belong to the separate private application. This repository explains the integration boundary without exposing private dashboard source code or endpoints.

## Open-source scope and brand

The Engine source and documentation in this repository are distributed under the [MIT License](LICENSE), subject to the license text. See [Open-source scope and responsibility](docs/OPEN_SOURCE_SCOPE.md) for a plain-language explanation of self-managed deployments, support boundaries, and the license's warranty disclaimer. The separate hosted service and its private source code are not included. **The Agata Proxima name, orbit logo, and other brand artwork are reserved brand assets and are not granted for reuse by the Engine source license.** See [COPYRIGHT.md](COPYRIGHT.md) for the scope and limitations of these statements. A source-code license cannot promise that every form of copying is technically preventable; trademark and copyright enforcement depend on applicable law.

## Contribute

Bug reports and feature requests: [GitHub Issues](https://github.com/Agata-Proxima/proxima-engine/issues). Before filing, review [Operations](docs/OPERATIONS.md) and redact secrets, tenant data, and private logs. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and the [Code of Conduct](CODE_OF_CONDUCT.md).

---

<p align="center">Built by Agata Proxima · <a href="https://agataproxima.com">agataproxima.com</a> · <a href="https://github.com/Agata-Proxima">GitHub organization</a></p>
