#!/usr/bin/env python3
"""Agata Proxima Engine CLI. Python 3.10+, standard library only."""
from __future__ import annotations
import argparse
import hashlib
import hmac
import json
import os
from pathlib import Path
import secrets
import sys
import time
import urllib.error
import urllib.request
import uuid

DEFAULT_CONFIG = Path(".proxima-dev/dev-config.json")

def load(path: Path) -> dict:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError) as exc:
        raise ValueError(f"Cannot load {path}: {exc}") from exc
    if data.get("format") != "proxima-local-dev-v1":
        raise ValueError("Unsupported local config format")
    return data

def init(args):
    directory = args.directory
    directory.mkdir(parents=True, exist_ok=True)
    config_path = directory / "dev-config.json"
    identity_path = directory / "node-identity.json"
    if config_path.exists() or identity_path.exists():
        raise ValueError("Local identity already exists; refusing to overwrite credentials")
    identity = {"node_id": str(uuid.uuid4()), "organization_id": "local-dev", "signing_key": secrets.token_hex(32)}
    config = {"format": "proxima-local-dev-v1", "mode": "local-only",
              "node_identity_path": str(identity_path.resolve()), "node_id": identity["node_id"],
              "organization_id": identity["organization_id"], "node_signing_key_hex": identity["signing_key"],
              "tenant_signing_key": secrets.token_hex(32),
              "listen_addr": "127.0.0.1:6432", "upstream_addr": "127.0.0.1:5432"}
    for path, data in ((identity_path, identity), (config_path, config)):
        with path.open("x", encoding="utf-8") as handle:
            json.dump(data, handle, indent=2)
            handle.write("\n")
        if os.name != "nt":
            path.chmod(0o600)
    print(f"Created local credentials in {directory}; keep them private. No Cloud account needed.")
    print("Run: python scripts/agata.py token --tenant tenant_a")
    print("Set PROXIMA_TENANT_SIGNING_KEY in the Engine to the literal tenant_signing_key in dev-config.json.")

def valid_tenant(tenant):
    return 1 <= len(tenant) <= 128 and all(c.isascii() and (c.isalnum() or c in "_-") for c in tenant)

def sign(key: str, tenant: str, expiry: int) -> str:
    if not valid_tenant(tenant):
        raise ValueError("Invalid tenant ID: use 1-128 ASCII letters, digits, underscore or hyphen")
    if len(key.encode("utf-8")) < 32:
        raise ValueError("Signing key must be at least 32 bytes")
    payload = f"v1.{tenant}.{expiry}"
    signature = hmac.new(key.encode("utf-8"), payload.encode("ascii"), hashlib.sha256).hexdigest()
    return f"{payload}.{signature}"

def token(args):
    if not 1 <= args.ttl <= 86400:
        raise ValueError("TTL must be 1-86400 seconds")
    config = load(args.config)
    print(sign(config["tenant_signing_key"], args.tenant, int(time.time()) + args.ttl))

def verify(args):
    config = load(args.config)
    parts = args.token.split(".")
    if len(parts) != 4 or parts[0] != "v1":
        raise ValueError("Invalid token format")
    try:
        expiry = int(parts[2])
    except ValueError as exc:
        raise ValueError("Invalid token expiry") from exc
    expected = sign(config["tenant_signing_key"], parts[1], expiry)
    if not hmac.compare_digest(expected, args.token):
        raise ValueError("Invalid tenant token signature")
    if expiry <= int(time.time()):
        raise ValueError("Tenant token expired")
    print(f"Valid local tenant token for {parts[1]} (expires {expiry})")

def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": "agata-engine-cli/0.1"})
    with urllib.request.urlopen(req, timeout=5) as response:
        return response.read(1024 * 1024).decode("utf-8")

def status(args):
    print(get(args.url.rstrip("/") + "/api/status"))

def doctor(args):
    errors = []
    for name, url in (("metrics health", args.metrics.rstrip("/") + "/healthz"),
                      ("local status", args.status.rstrip("/") + "/api/status")):
        try:
            body = get(url)
            print(f"PASS {name}: {body[:200]}")
        except (OSError, urllib.error.URLError, ValueError) as exc:
            errors.append(name)
            print(f"FAIL {name}: {exc}", file=sys.stderr)
    if errors:
        raise ValueError("Engine health checks failed: " + ", ".join(errors))

def main(argv=None):
    parser = argparse.ArgumentParser(prog="agata", description="Agata Proxima Engine local CLI (not a hosted management CLI)")
    parser.add_argument("--version", action="version", version="agata-engine-cli 0.1.0")
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("init", help="initialize disposable local node credentials")
    p.add_argument("--directory", type=Path, default=Path(".proxima-dev"))
    p.set_defaults(func=init)
    p = sub.add_parser("token", help="mint signed local tenant context")
    p.add_argument("--config", type=Path, default=DEFAULT_CONFIG)
    p.add_argument("--tenant", required=True)
    p.add_argument("--ttl", type=int, default=300)
    p.set_defaults(func=token)
    p = sub.add_parser("verify-token", help="verify a local tenant token offline")
    p.add_argument("--config", type=Path, default=DEFAULT_CONFIG)
    p.add_argument("token")
    p.set_defaults(func=verify)
    p = sub.add_parser("status", help="read running Engine status")
    p.add_argument("--url", default="http://127.0.0.1:9080")
    p.set_defaults(func=status)
    p = sub.add_parser("doctor", help="check Engine health and status")
    p.add_argument("--metrics", default="http://127.0.0.1:9090")
    p.add_argument("--status", default="http://127.0.0.1:9080")
    p.set_defaults(func=doctor)
    args = parser.parse_args(argv)
    try:
        args.func(args)
    except (ValueError, OSError, KeyError, urllib.error.URLError) as exc:
        print(f"agata: {exc}", file=sys.stderr)
        return 1
    return 0

if __name__ == "__main__":
    sys.exit(main())
