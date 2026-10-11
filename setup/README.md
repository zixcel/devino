# Explicit model conversion setup

This Python environment converts BERT through maintained Transformers, PyTorch
and OpenVINO. Old TensorFlow/Intel TensorFlow/MKL development dependencies were
not imported by this implementation and are no longer installed here. GPU driver
installation and system package changes are separate operator tasks.

Python 3.12 or 3.13 is required. Use the committed uv.lock with `uv sync --frozen`
after the applicable build/storage gate passes. This does not install GPU drivers.
Actual CPU/GPU conversion and inference acceptance remain required.

Set MODEL_ID, MODEL_REVISION (exact 40-character commit SHA), MODEL_DEVICE,
MODEL_OUTPUT (absolute .xml path), and MODEL_TEXT, then run
`uv run --frozen python src/setup/bert_base_uncase.py`.
The loader disallows remote Python execution and requires safetensors weights.
Imports do not download/load models. Existing XML/BIN outputs are not overwritten.
Model text is local inference input and is excluded from configuration repr.

Run backend-free configuration tests with
`python3 -B -m unittest discover -s tests`.
