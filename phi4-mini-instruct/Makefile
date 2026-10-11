SHELL := /bin/bash

.PHONY: help install-dependencies update-dependencies load inference-onnx inference-openvino

help:	## Show this help.
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*##.*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

install-dependencies: ## Install Project Dependencies
	@uv sync --frozen

update-dependencies: ## Update Project Dependencies
	@uv lock

-include ../.env
inference-onnx: ## Run local ONNX GenAI inference; MODEL_DIR and MODEL_EXECUTION_PROVIDER required.
	uv run --frozen python src/phi4_mini_instruct/inference_onnxruntime.py

inference-openvino: ## Run local OpenVINO GenAI inference; MODEL_DIR and MODEL_DEVICE required.
	uv run --frozen python src/phi4_mini_instruct/inference_openvino.py
