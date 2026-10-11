

## Current local environment

Python 3.12/3.13 and `uv sync --frozen` use the committed uv.lock.
Only the two native inference backends are installed here. Model export and
GPU/driver provisioning are separate environments and remain subject to their
own compatibility and security checks.

Use an existing absolute MODEL_DIR. OpenVINO also requires MODEL_DEVICE; ONNX
requires MODEL_EXECUTION_PROVIDER (`cpu`, `cuda`, or `dml`), or the equivalent
CLI options. The Make targets no longer select a model or hardware implicitly.
Backend hardware/model acceptance for the updated versions remains pending.
