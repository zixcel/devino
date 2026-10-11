

## Current export dependency environment

Use Python 3.12/3.13 and `uv sync --frozen` with the committed uv.lock.
The unpinned Git dependencies, nightly OpenVINO registry, local PyTorch wheel
and unused TensorFlow packages were replaced by compatible official PyPI
releases. Model export remains blocked for publication/production until
Transformers advisories and exact model/hardware acceptance are resolved.
The compatible export profile pins Transformers 5.5.4; this is not equivalent
to the separate setup environment using Transformers 5.19.0. Legacy model
scripts still require review of remote-code trust, revisions and device settings.
Set MODEL_ID explicitly; the Makefile does not select a model.
