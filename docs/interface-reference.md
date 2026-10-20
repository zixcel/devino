# Rust interface

| API | Responsibility |
| --- | --- |
| `NativeLibraries` | Hash two explicit regular entry libraries; the caller admits their full dependency/plugin closure |
| `OpenVinoRuntime::load` | Require a protected worker, load one distribution per process; reject mixed/partially loaded selections |
| `OpenVinoRuntime::devices` | Observe available native device names without selecting GPU or automatic fallback |
| `OpenVinoCpu::load` | Revalidate a complete OpenVINO IR/tokenizer bundle and load on CPU with one thread/stream |
| `GenerationOptions` | Bound prompt bytes, generated/total tokens, output bytes and elapsed budget |
| `GenerationControl` | Monotonic deadline and shared terminal cancellation |
| `GenerationObservation` | Backend-local text and actual native token counts; not application/runtime authorization |

`GenerationOptions` accepts prompts of 1–4096 UTF-8 bytes without NUL, 1–256
new tokens, total-token limit up to 8192, 1–65536 output bytes and 1–30000 ms.
Stream fragments remain provisional; limit or cancellation errors discard the
result. A native callback is cooperative, so the owner must enforce a process
termination deadline even during pipeline construction or a blocked native call.

OpenVINO bundles require the model, tokenizer and detokenizer XML/BIN pairs,
config.json and tokenizer_config.json in the complete manifest. Manifest identity
is checked by zixcel-local-inference, not derived from a mutable model name.
A hash observation does not protect writable paths against concurrent changes;
callers must separately protect admitted model/runtime directories.

The SDK uses Intel-maintained `openvino`, `openvino-sys` and `openvino-genai`
bindings pinned to 0.11.1. Runtime-linking permits source checks without native
libraries. ABI/plugin/library compatibility still needs the explicit native
consumer acceptance for each claimed supported combination.

Historical OVMS installation recipes do not constitute an implemented server.
GPU/ONNX execution and PyTorch export are not supported by this Rust interface.
