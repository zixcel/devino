# devino interface reference

Use the [usage guide](getting-started.md) for the first steps. This reference preserves the current interface details and operational limits. Run command examples from the repository root, after preparing the exact declared dependencies and registered configuration.

## Repository Structure

- [setup](../setup): Provides scripts for setting up a Pytorch-XPU environment using Ubuntu 22 and Poetry.
installation.
- Model Directories: Named according to Hugging Face model IDs, each containing conversion and inference scripts based on the setup environment.
- [playground](../playground): Contains sample scripts tested in an OpenVINO 2025 and Ubuntu 24 environment.

## Key Features

- **OpenVINO IR Conversion**: Converts Hugging Face models to OpenVINO IR format for optimized inference.
- **OpenVINO Model Server (OVMC)**: Implements an OpenVINO model server for running converted models.
- **GPU Acceleration**: Provides performance improvements for inference using Intel GPUs.

## Getting Started

1. **Verify Ubuntu Compatibility**: Check the appropriate WSL Ubuntu version using [intel-gpu-wsl-advisor](https://github.com/zixcel/intel-gpu-wsl-advisor).
  - The advisor is optional; callers may verify the Windows driver, WSL kernel and runtime requirements directly.
2. **Setup Environment**: Use the scripts in `setup/` to install dependencies and configure Pytorch-XPU.
3. **Convert Models**: Run the provided conversion scripts to transform models into OpenVINO IR format.
4. **Deploy Model Server**: Install OpenVINO GenAI's OVMC server and execute converted models. using the checked-in experiment scripts

## References

- [OpenVINO Documentation (2025)](https://docs.openvino.ai/2025/index.html)
- [Hugging Face Optimum-Intel](https://huggingface.co/blog/deploy-with-openvino)

This repository is under active development, integrating new features for optimized inference and deployment on Intel hardware.

## Registered local inputs

Some historical setup recipes use a local PyTorch wheel. Supply the selected, verified wheel at `registration/torch.whl` before running such a recipe; registration data and downloaded model weights are excluded from Git. External model/runtime licenses and hardware compatibility must be verified for the selected experiment. The migration validates source and configuration without downloading models, starting servers, or changing the host.
