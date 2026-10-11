"""Explicit, import-safe BERT to OpenVINO conversion using safetensors only.

API reference retained from the original example:
https://docs.openvino.ai/2025/openvino-workflow/model-preparation.html
"""
from dataclasses import dataclass, field
from pathlib import Path
import os
import re
from typing import Mapping


@dataclass(frozen=True)
class ModelConfig:
    model_id: str
    revision: str
    device: str
    output: Path
    text: str = field(repr=False)

    @classmethod
    def from_env(cls, values: Mapping[str, str]) -> "ModelConfig":
        names = ("MODEL_ID", "MODEL_REVISION", "MODEL_DEVICE", "MODEL_OUTPUT", "MODEL_TEXT")
        for name in names:
            if not values.get(name, "").strip():
                raise ValueError(f"{name} is required")
        revision = values["MODEL_REVISION"]
        if not re.fullmatch(r"[0-9a-f]{40}", revision):
            raise ValueError("MODEL_REVISION must be an immutable commit SHA")
        output = Path(values["MODEL_OUTPUT"])
        if not output.is_absolute() or output.suffix != ".xml":
            raise ValueError("MODEL_OUTPUT must be an absolute XML file path")
        return cls(values["MODEL_ID"], revision, values["MODEL_DEVICE"], output, values["MODEL_TEXT"])


def convert(config: ModelConfig) -> None:
    # Reject collisions before downloads or loading any third-party backend.
    if config.output.exists() or config.output.with_suffix(".bin").exists():
        raise FileExistsError("Model output already exists")
    from transformers import BertTokenizer, BertModel
    import openvino as ov

    tokenizer = BertTokenizer.from_pretrained(
        config.model_id, revision=config.revision, trust_remote_code=False
    )
    model = BertModel.from_pretrained(
        config.model_id, revision=config.revision,
        trust_remote_code=False, use_safetensors=True
    )
    model.eval()
    encoded = tokenizer(config.text, return_tensors="pt")
    converted = ov.convert_model(model, example_input=dict(encoded))
    config.output.parent.mkdir(parents=True, exist_ok=True)
    ov.save_model(converted, config.output)
    compiled = ov.compile_model(converted, config.device)
    compiled(dict(encoded))


def main() -> None:
    convert(ModelConfig.from_env(os.environ))


if __name__ == "__main__":
    main()
