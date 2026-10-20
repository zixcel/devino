# Model preparation and conversion alternatives

The Python/PyTorch export experiments are retired by the owner's 2026-10-20
scope decision. Optimum Intel 2.2.0 requires Transformers >=4.51,<5.6, which
excludes the reviewed advisory-patched range. We did not override constraints,
ignore advisories, replace an exporter with an untested rewrite, or add a
new Python environment. The removed lockfiles are no longer product inputs.

## Selected paths

| Input/use | Path | Status/limits |
| --- | --- | --- |
| Text-generation pipeline | Reviewed preconverted OpenVINO GenAI model/tokenizer/config bundle | Implemented Rust CPU adapter; real native consumer tested with synthetic weights |
| Compatible conventional ONNX model | Intel Rust `Core::read_model_from_buffer` or `read_model_from_file`, then CPU compile/infer | Official runtime reads ONNX without writing XML/BIN; this does not create a GenAI tokenizer/LLM pipeline |
| Arbitrary PyTorch/Hugging Face source | Separate upstream preparation process, then admit resulting immutable bundle | No in-product source conversion or remote-code execution; not claimed as supported |

Preconverted models do not remove review obligations: select an immutable
revision, license, exact complete file set and hashes; separately admit the
native distribution/plugin closure. A mutable model name is not an identity.

## Rust conversion assessment

Intel maintains the `openvino` Rust bindings used here. Version 0.11.1 exposes
model reading, compilation and tensor inference. Its public safe `Model` API
does not expose IR XML/BIN serialization. Reading ONNX performs the runtime
frontend conversion in memory; it is not an on-disk OpenVINO IR export.
The C API's compiled-model export, where available, produces a device/runtime
compiled representation, not interchangeable IR XML/BIN. These formats must
not be relabelled as equivalent.

OpenVINO's supported `convert_model` / `save_model` / `ovc` preparation APIs
are upstream Python tooling; C++ also exposes serialization. Wrapping those
through custom unsafe FFI, inventing a Rust PyTorch converter or shipping a
second Python environment would add a new maintenance and security boundary.
We do not add it merely to preserve an experimental command. If IR export
later becomes necessary, evaluate a vendor-maintained safe Rust serialization
API or a separately admitted vendor converter, and test numeric equivalence,
external weights, unsupported operations and output identity before support.

Sources checked on 2026-10-20:

- [Intel Rust bindings](https://github.com/intel/openvino-rs), especially
  `crates/openvino/src/core.rs` and `model.rs` in published 0.11.1.
- [OpenVINO ONNX frontend](https://docs.openvino.ai/2026/openvino-workflow/model-preparation/convert-model-onnx.html).
- [OpenVINO model preparation](https://docs.openvino.ai/2026/openvino-workflow/model-preparation.html).
- [Optimum Intel metadata](https://pypi.org/pypi/optimum-intel/2.2.0/json).

Revisit when Intel exposes a supported safe Rust IR writer or when a concrete
consumer needs an on-disk conversion that direct ONNX loading cannot satisfy.
The explicit native consumer also loads an original static F32[2] ONNX
Identity graph from bytes and verifies two numeric input/output pairs through
the real OpenVINO CPU plugin. This demonstrates direct ONNX loading, not
on-disk IR export or general ONNX/operator/model support. No export capability
is inferred from a passing compilation-only test.
