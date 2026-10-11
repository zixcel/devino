#!/usr/bin/env bash
# Local project setup only. Operators provision GPU drivers separately.
# https://www.intel.com/content/www/us/en/docs/oneapi/installation-guide-linux/2025-0/configure-wsl-2-for-gpu-workflows.html
set -euo pipefail
project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
command -v uv >/dev/null || { echo "Install a verified uv release before running setup." >&2; exit 1; }
uv lock --project "$project_dir" --check --no-build --no-python-downloads
uv sync --project "$project_dir" --frozen --no-python-downloads
printf '%s\n' "Local environment ready. Configure the MODEL_* settings documented in README.md."
