# Agata CLI (public Engine)

This is a real local Engine command-line tool implemented with Python 3.10+ standard library. It is **not** an npm package or the separate hosted Cloud CLI.

From the repository root:

```bash
python scripts/agata.py --help
python scripts/agata.py init
python scripts/agata.py token --tenant tenant_a --ttl 300
python scripts/agata.py doctor
python scripts/agata.py status
```

For Windows PowerShell, use the same commands with `python`. On Unix, `python3` may be required.

`init` creates local credentials in `.proxima-dev/` and refuses to overwrite existing credentials. Configure the Engine's `PROXIMA_TENANT_SIGNING_KEY` using the literal `tenant_signing_key` from `.proxima-dev/dev-config.json`; never publish it. The CLI does not automatically modify running containers or register a hosted node.

`verify-token` accepts a generated token as a positional argument, but remember shell command-line arguments may appear in process listings; do not use this for sensitive production credentials.

`doctor` and `status` read only local Engine HTTP endpoints. Neither proves tenant isolation. For positive/negative database isolation checks, run [the tenant isolation lab](TENANT_ISOLATION.md) against a disposable PostgreSQL database.

Test locally with `python -m unittest discover -s tests_cli -v`.
