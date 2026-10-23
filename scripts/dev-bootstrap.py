#!/usr/bin/env python3
"""Create disposable, self-managed Proxima Engine developer credentials.

Uses only Python's standard library. It does not contact Agata Proxima Cloud.
"""
from __future__ import annotations

import argparse
import hashlib
import hmac
import json
import secrets
import time
import uuid
from pathlib import Path

def write_private(path: Path, data: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(data, encoding="utf-8")
    try:
        path.chmod(0o600)
    except OSError:
        # Windows does not implement POSIX mode bits; protect the directory with
        # normal Windows user-only file permissions.
        pass

def load_config(path: Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise SystemExit(f"Cannot read config {path}: {exc}") from exc
    if value.get("format") != "proxima-local-dev-v1":
        raise SystemExit("Unsupported config format; re-run the init command with a new directory.")
    return value

def init(args: argparse.Namespace) -> None:
    root = args.directory.resolve()
    root.mkdir(parents=True, exist_ok=True)
    identity = {
        "node_id": str(uuid.uuid4()),
        "organization_id": "local-dev",
        "signing_key": secrets.token_hex(32),
    }
    tenant_key = secrets.token_hex(32)
    config = {
        "format": "proxima-local-dev-v1",
        "mode": "local-only",
        "node_identity_path": str((root / "node-identity.json").resolve()),
        "node_id": identity["node_id"],
        "organization_id": identity["organization_id"],
        "node_signing_key_hex": identity["signing_key"],
        "tenant_signing_key": tenant_key,
        "listen_addr": "127.0.0.1:6432",
        "upstream_addr": "127.0.0.1:5432",
        "warning": "Disposable local development credentials. Never use in production.",
    }
    write_private(root / "node-identity.json", json.dumps(identity, indent=2) + "\n")
    write_private(root / "dev-config.json", json.dumps(config, indent=2) + "\n")
    # Keep generated secrets out of Git without overwriting existing ignore rules.
    gitignore = Path.cwd() / ".gitignore"
    ignore_line = ".proxima-dev/"
    existing = gitignore.read_text(encoding="utf-8") if gitignore.exists() else ""
    if ignore_line not in {line.strip() for line in existing.splitlines()}:
        separator = "" if not existing or existing.endswith("\n") else "\n"
        gitignore.write_text(existing + separator + ignore_line + "\n", encoding="utf-8")
    print("Local developer credentials created.")
    print(f"Config:   {root / 'dev-config.json'}")
    print(f"Identity: {root / 'node-identity.json'}")
    print(f"Node ID:  {identity['node_id']}")
    print()
    print("No hosted account, API call, subscription, or dashboard was used.")
    print("Keep this directory private; it contains signing secrets.")
    print("Next: use the generated config to set the Engine environment, then run:")
    print(f'  python scripts/dev-bootstrap.py token --config "{root / "dev-config.json"}" --tenant tenant_a --ttl 300')

def token(args: argparse.Namespace) -> None:
    config = load_config(args.config)
    tenant = args.tenant
    if not tenant or len(tenant) > 128 or not all(c.isascii() and (c.isalnum() or c in "_-") for c in tenant):
        raise SystemExit("Tenant ID must be 1–128 ASCII letters, digits, underscores, or hyphens.")
    if not 1 <= args.ttl <= 86400:
        raise SystemExit("TTL must be between 1 and 86400 seconds for local development.")
    secret = bytes.fromhex(config["tenant_signing_key"])
    expires = int(time.time()) + args.ttl
    payload = f"v1.{tenant}.{expires}"
    signature = hmac.new(secret, payload.encode("utf-8"), hashlib.sha256).hexdigest()
    print(f"{payload}.{signature}")

def main() -> None:
    parser = argparse.ArgumentParser(description="Bootstrap local-only Proxima developer credentials.")
    sub = parser.add_subparsers(dest="command", required=True)
    init_parser = sub.add_parser("init", help="generate local node identity and tenant signing key")
    init_parser.add_argument("--directory", type=Path, default=Path(".proxima-dev"), help="private output directory (default: .proxima-dev)")
    init_parser.set_defaults(func=init)
    token_parser = sub.add_parser("token", help="mint a short-lived local tenant-context token")
    token_parser.add_argument("--config", type=Path, default=Path(".proxima-dev/dev-config.json"))
    token_parser.add_argument("--tenant", required=True, help="tenant ID")
    token_parser.add_argument("--ttl", type=int, default=300, help="token lifetime in seconds (default: 300)")
    token_parser.set_defaults(func=token)
    args = parser.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()
