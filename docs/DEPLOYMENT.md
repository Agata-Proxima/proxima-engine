# Deploy Proxima Engine

This guide covers building the Engine container and deploying it beside an existing PostgreSQL service. It does not deploy a hosted control plane.

## 1. Build the image

From the repository root:

```bash
docker build -t agata-proxima-engine:local .
```

The image uses a multi-stage Rust build, runs as a non-root user, and includes the logo and local status page embedded in the binary.

## 2. Run against a reachable PostgreSQL server

The Engine must be able to reach the configured upstream from inside its container. Replace the example address with a DNS name or IP reachable from the container:

```bash
docker run --rm --name proxima-engine \
  -p 127.0.0.1:6432:6432 \
  -e PROXIMA_LISTEN_ADDR=0.0.0.0:6432 \
  -e PROXIMA_UPSTREAM_ADDR=host.docker.internal:5432 \
  -e PROXIMA_DASHBOARD_ENABLED=false \
  -e PROXIMA_METRICS_LISTEN_ADDR=127.0.0.1:9090 \
  -v proxima-engine-data:/var/lib/proxima \
  agata-proxima-engine:local
```

This example is for local testing. On Linux, `host.docker.internal` may need to be enabled with `--add-host=host.docker.internal:host-gateway`. In production, prefer a private service network and an explicit upstream hostname. Do not expose PostgreSQL to the public internet.

If Prometheus needs to scrape metrics, bind the metrics listener to the container interface and publish it only on a private monitoring network. Do not expose metrics or the local dashboard to the public internet without a separate access-control layer.

## 3. Configure tenant enforcement

For tenant-aware routing, set `PROXIMA_TENANT_SIGNING_KEY` to a strong secret of at least 32 bytes and create the matching PostgreSQL roles and RLS policies. Once configured, the Engine requires a valid signed tenant token at startup; missing, expired, malformed, or invalid tokens are rejected.

The trusted application backend must sign tenant tokens after verifying the user's access to that tenant. Never ship the signing key to a browser, mobile client, or public repository. Follow [Tenant isolation](TENANT_ISOLATION.md).

## 4. Enable TLS on both legs

Client-facing TLS and upstream TLS are independent. Example environment values (replace paths and hostnames with your deployment's real values):

```bash
PROXIMA_TLS_CERT_FILE=/run/secrets/proxima-server.crt
PROXIMA_TLS_KEY_FILE=/run/secrets/proxima-server.key
PROXIMA_TLS_REQUIRE_CLIENT=true
PROXIMA_UPSTREAM_TLS_MODE=verify-full
PROXIMA_UPSTREAM_TLS_CA_FILE=/run/secrets/postgres-ca.crt
PROXIMA_UPSTREAM_TLS_SERVER_NAME=postgres.internal.example
```

The certificate and key must be mounted read-only and readable by the Engine process. When upstream TLS is enabled, provide a trusted CA; `verify-full` also requires the expected server name. Test certificate rotation and expired/incorrect certificates before production use.

## 5. Persistent state and node enrollment

Mount a persistent, access-restricted volume at `/var/lib/proxima`. The container defaults store node identity, the local attestation ledger, and break-glass replay records there. Back up state according to your security and recovery requirements.

If you register the Engine with a compatible control plane, configure `PROXIMA_NODE_ID`, `PROXIMA_NODE_ENROLLMENT_URL`, and `PROXIMA_NODE_ENROLLMENT_TOKEN` for first enrollment. The one-time token is consumed during bootstrap; the saved identity is used on later restarts. See [Node enrollment](NODE_ENROLLMENT.md).

## 6. Production checklist

- [ ] Pin the image to a reviewed release or immutable image digest.
- [ ] Replace every local/demo password and secret.
- [ ] Keep the Engine and PostgreSQL on private networks with firewall rules allowing only required traffic.
- [ ] Enable client TLS and upstream TLS with certificate validation.
- [ ] Enable tenant-token enforcement and test cross-tenant read/write denial.
- [ ] Use least-privilege PostgreSQL roles; enforce RLS where required; remove unintended public grants.
- [ ] Keep signing keys and identity files in a secret store or restricted persistent volume.
- [ ] Keep dashboard and metrics endpoints private.
- [ ] Configure log collection, alerts, capacity limits, and connection pool limits.
- [ ] Test restart, upstream outage, token expiry, certificate expiry, backup, and recovery.
- [ ] Run [Verification](VERIFICATION.md) against the exact image and environment.

## Deployment boundary

The Engine is a data-plane process, not a complete hosted service. Database provisioning, secret distribution, orchestration, ingress, load balancing, control-plane registration, backups, monitoring infrastructure, and incident response remain deployment responsibilities. Do not infer production readiness from a successful image build alone.


## Production acceptance checklist

Before directing real application traffic to an Engine deployment, record evidence for each item:

- [ ] Pin the reviewed source revision and container image digest.
- [ ] Replace sample credentials and store secrets outside the image and repository.
- [ ] Restrict PostgreSQL, proxy, status, and metrics listeners to their intended callers.
- [ ] Enable verified TLS for production connections wherever required by the network trust model.
- [ ] Configure least-privileged PostgreSQL roles and test RLS with two synthetic tenants.
- [ ] Confirm tenant-token signing happens only in a trusted backend and rotate the signing secret safely.
- [ ] Persist and protect node identity and attestation-related state according to the configuration guide.
- [ ] Test startup, upstream outage, restart, secret rotation, backup/restore, and rollback procedures.
- [ ] Set alerts and verify that operators can access sanitized logs and metrics.
- [ ] Document an incident owner and a way to disable or isolate the deployment.

Do not mark an item complete because a sample configuration contains the setting. Verify the effective runtime configuration and record the test result. This checklist is operational guidance, not a certification or guarantee of production readiness.

For hosted account, organization, or plan workflows, use [Cloud boundary](CLOUD_CONTROL_PLANE.md); this deployment guide covers the Engine only.
