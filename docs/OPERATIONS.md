# Operations and troubleshooting

## Health and status

Default local endpoints:

- Engine status page: `http://127.0.0.1:9080/`
- Status JSON: `http://127.0.0.1:9080/api/status`
- Dashboard health: `http://127.0.0.1:9080/health`
- Metrics and health: `http://127.0.0.1:9090/metrics`, `/health`, and `/healthz`

The embedded status page and metrics listener do not authenticate callers. Keep them on loopback or a restricted private network. A health response means the HTTP listener is answering; it does not confirm that PostgreSQL is reachable.

## Inspect logs

For the local Compose profile:

```bash
docker compose -f compose.dev.yaml ps
docker compose -f compose.dev.yaml logs --tail=200 engine
docker compose -f compose.dev.yaml logs --tail=100 postgres
```

For a native process, keep the startup terminal open and use the error message to identify invalid settings, upstream connection failures, TLS errors, or enrollment failures. Do not paste real tokens, passwords, private keys, or customer data into issue reports.

## Common failures

| Symptom | Likely cause | What to check |
| --- | --- | --- |
| Engine exits on startup | Invalid environment value or incomplete TLS configuration | Read the first startup error; check [Configuration](CONFIGURATION.md) |
| Connection refused on 6432 | Engine is stopped or bound to a different address | Check process/container health and `PROXIMA_LISTEN_ADDR` |
| Upstream connection error | Database is down or unreachable from the Engine network | Check `PROXIMA_UPSTREAM_ADDR`, DNS, firewall, and database health |
| Tenant token required | `PROXIMA_TENANT_SIGNING_KEY` is configured, so tenant context is enforced | Send a valid, signed token in PostgreSQL startup options |
| Invalid/expired tenant token | Wrong signing key, token signature, tenant ID, or expiry | Sign with the exact configured key; generate a fresh short-lived token |
| PostgreSQL permission denied | Wrong tenant role password, grants, or RLS policy | Confirm the role mapped from the tenant ID and inspect RLS grants |
| TLS handshake failure | Missing certificate/key, untrusted CA, or incorrect server name | Check both TLS legs and use the real hostname; do not disable verification |
| Enrollment returns 401 | Token expired, consumed, or not paired with the node ID | Obtain a valid one-time token from the compatible control plane |
| Status endpoint returns 404 | Wrong path or listener | Use the paths listed above; the Engine is not a general REST API |
| No attestation records | Attestation is disabled or signing configuration is incomplete | Check the signing key, `PROXIMA_ATTESTATION_P99_OVERHEAD_MS`, and ledger path |

## Restart and persistence

The Engine keeps attestation records, node identity, and emergency-token replay markers on disk. In containers, mount a persistent, restricted volume at `/var/lib/proxima` as shown in the Dockerfile. Do not delete the volume during an ordinary restart.

The local development Compose profile keeps PostgreSQL data in a named volume. `docker compose down` stops/removes containers but keeps volumes; `docker compose down -v` deletes them.

## Monitoring practices

- Scrape metrics from a private monitoring network.
- Alert on repeated session failures, rejected connections, active connection saturation, upstream errors, and stale attestation delivery.
- Treat process-local metrics as telemetry; they reset on restart and are not an audit trail.
- Protect and back up the attestation ledger and identity file.
- Test restore, certificate rotation, secret rotation, upstream outages, and planned restarts before production.


## Symptom-to-check guide

| Symptom | Check first | Avoid |
|---|---|---|
| Health endpoint unavailable | Process state, bind address, port mapping, container logs | Opening the listener to every network as a quick fix |
| Health is green but SQL fails | Upstream reachability, database name, credentials, TLS mode, protocol listener | Treating HTTP health as a successful database test |
| SQL works but tenant rows are wrong | Token issuer, tenant ID, selected database role, RLS policies | Granting superuser or bypass-RLS privileges to the application |
| Enrollment rejected | Token expiry, node ID, expected control-plane configuration, outbound network | Reusing or publishing one-time enrollment tokens |
| Metrics missing | Metrics listener, scrape target, network policy, scrape errors | Publishing metrics unauthenticated on a public interface |
| Attestation or break-glass verification fails | Exact verifier options, trusted key, organization/node context, expiry, clock | Disabling verification without an approved incident procedure |

## Incident handling

1. Protect users first: isolate the affected path or stop sensitive traffic if tenant separation is uncertain.
2. Preserve relevant sanitized logs and the exact Engine commit/image digest.
3. Do not copy live secrets or customer data into issues, chat, or email.
4. Reproduce with synthetic tenants and a disposable database when possible.
5. Rotate credentials only through the established secret-management process; coordinate dependent services.
6. Validate recovery with both successful and denied requests before restoring traffic.
7. Record cause, impact, mitigation, and follow-up tests.

Counters and process metrics are operational telemetry, not an immutable audit record. Review the specific persistence and retention behavior of the relevant feature before relying on it as evidence.
