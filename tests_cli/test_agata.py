"""Regression tests for the public Engine CLI; no network required."""
import contextlib
import hashlib
import hmac
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CLI = ROOT / "scripts" / "agata.py"

class CliTests(unittest.TestCase):
    def run_cli(self, *args):
        return subprocess.run([sys.executable, str(CLI), *map(str, args)],
                              cwd=ROOT, text=True, capture_output=True)

    def test_init_token_verify_and_tampering(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp) / ".proxima-dev"
            self.assertEqual(self.run_cli("init", "--directory", directory).returncode, 0)
            self.assertNotEqual(self.run_cli("init", "--directory", directory).returncode, 0)
            cfg = directory / "dev-config.json"
            config = json.loads(cfg.read_text())
            token = self.run_cli("token", "--config", cfg, "--tenant", "tenant_a", "--ttl", "300")
            self.assertEqual(token.returncode, 0, token.stderr)
            value = token.stdout.strip()
            parts = value.split(".")
            self.assertEqual(len(parts), 4)
            payload = ".".join(parts[:3])
            expected = hmac.new(config["tenant_signing_key"].encode(), payload.encode(), hashlib.sha256).hexdigest()
            self.assertEqual(parts[3], expected)
            self.assertEqual(self.run_cli("verify-token", "--config", cfg, value).returncode, 0)
            self.assertNotEqual(self.run_cli("verify-token", "--config", cfg, value.replace("tenant_a", "tenant_b")).returncode, 0)
            self.assertNotEqual(self.run_cli("token", "--config", cfg, "--tenant", "bad!").returncode, 0)

    def test_help(self):
        self.assertEqual(self.run_cli("--help").returncode, 0)

if __name__ == "__main__":
    unittest.main()
