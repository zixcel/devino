SHELL := /bin/bash

.PHONY: help install-dependencies update-dependencies load sanity-check inference


help:	## Show this help.
	@grep -hE '^[A-Za-z0-9_ \-]*?:.*##.*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

install-dependencies: ## Install Project Dependencies
	@uv sync --frozen

update-dependencies: ## Update Project Dependencies
	@uv lock

-include ../.env
load: ## Load model from HugggingFace
	#uv run --frozen optimum-cli export openvino --task text-generation --cache_dir "${HF_HOME}/hub" --model "${MODEL_ID}" --trust-remote-code "${OV_HOME}/microsoft--Phi-4-mini-instruct"
	# source /opt/intel/oneapi/setvars.sh --force &&\
	HF_HOME=${HF_HOME} \
	OV_HOME=${OV_HOME} \
	MODEL_ID=${MODEL_ID} \
	TF_ENABLE_ONEDNN_OPTS=1 \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:${OVMS_LD_LIBRARY_PATH}:/lib:/usr/lib:/usr/lib/x86_64-linux-gnu \
	uv run --frozen python phi_4_mini_instruct/load.py

-include ../.env
inference: ## Inference model from Openvino IR
	HF_HOME=${HF_HOME} \
	OV_HOME=${OV_HOME} \
	MODEL_ID=${MODEL_ID} \
	TF_ENABLE_ONEDNN_OPTS=0 \
	LD_LIBRARY_PATH=${LD_LIBRARY_PATH} \
	uv run --frozen python phi_4_mini_instruct/inference.py



sanity-check:
	uv run --frozen python -c "import torch; import intel_extension_for_pytorch as ipex; print(torch.__version__); print(ipex.__version__); [print(f'[{i}]: {torch.xpu.get_device_properties(i)}') for i in range(torch.xpu.device_count())];"
