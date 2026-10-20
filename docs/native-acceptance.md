# Explicit CPU acceptance

The `native_cpu` example is an opt-in, real native-consumer check. Ordinary
`cargo test` does not download or activate a model. Missing native acceptance
is not a passing or skipped test: execute this command separately and retain
its exit status and output before claiming a supported native distribution.

```sh
cargo build --locked --example native_cpu
./target/debug/examples/native_cpu \
  /absolute/model-directory /absolute/manifest.json EXPECTED_MANIFEST_SHA256 \
  /absolute/native-libraries.json /absolute/runtime-libs:/absolute/tbb-libs
```

The operator must verify the complete native distribution, its dependencies,
model revision and license before invocation. `NativeLibraries` contains
absolute regular-file paths `openvinoC` / `genaiC` and their lowercase SHA256
values `openvinoSha256` / `genaiSha256`. Resolve distribution symlinks to their
regular targets when constructing this document. It is not a credential file.
The final argument contains only explicitly selected loader directories.
The launcher clears ambient environment variables before the worker starts.

The worker attaches a dedicated process group, binds owner death, disables
core collection, limits address space to 2 GiB and CPU time to 20 seconds.
The owner kills and reaps only its exact, unreaped child after 45 seconds.
CPU generation uses one inference thread and one stream. No GPU is selected,
no model code is executed, no download or host installation occurs here.

The case first runs an embedded static F32[2] ONNX Identity graph directly
through Intel’s Rust Core API and checks two numeric inputs exactly. It then
checks two equal eight-token generations, actual token usage,
pre-cancellation rejection, one-byte streaming output rejection and successful
subsequent generation. It does not prove arbitrary model compatibility,
mid-call cancellation latency, GPU support or long-term leak freedom.

## Observed combination (2026-10-20)

- Intel GenAI archive: `openvino_genai_ubuntu24_2026.4.1.0_x86_64.tar.gz`.
  SHA256: `b8ba7b6e5ae7899e63e5e3c7f9d022ad8a4207247521e46c507f6f3ef23ea2c5`.
  Source: [Intel distribution](https://storage.openvinotoolkit.org/repositories/openvino_genai/packages/2026.4.1/linux/).
- The archive's GenAI C library requires an OpenCL loader even for this CPU
  case. Ubuntu's `ocl-icd-libopencl1` 2.3.2-1build1 was downloaded, verified
  against package metadata and extracted locally, without installing it.
  Package SHA256: `6d98029b06335bc23166834719ac5e697bcb4a136b40eebc263dfe481e05839b`.
- Synthetic model: [dummy-cyclic-gpt2-ov](https://huggingface.co/mzeglars/dummy-cyclic-gpt2-ov),
  revision `5e45bfe8087902659ac48ff6847a2f242fa3de21`, Apache-2.0 declaration.
  All weights are synthetic; no model weights are distributed in this crate.
  This is a real native inference test with a synthetic model, not a language
  quality test. Expected eight-token prefix: `OpenVINO is an open-source`.
- Model manifest SHA256:
  `075023db8cf46150574e6183eb88c24d63530c60262619efbe08f1d73a0fd8f5`.
  The manifest is retained beside this document. Verify each upstream LFS hash
  and the immutable revision when obtaining the listed files.

The protected local run succeeded; the parent and WSL boot continued.
The first native-load attempt failed closed because the OpenCL loader was
missing. This is recorded as a failed attempt, not as inference success.
