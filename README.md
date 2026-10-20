# devino

A Rust OpenVINO CPU backend for explicitly verified local model bundles.

Devino owns native library loading, device observation and bounded generation.
[zixcel-local-inference](https://github.com/zixcel/zixcel-local-inference) owns
model identity, catalog/acquisition and runtime lifecycle. The caller owns the
worker process and authorization; Devino creates no second store or daemon.

## Rust interface

1. Admit the complete native distribution and model identity independently.
2. Spawn a dedicated Linux worker and attach `BackendProcessContext` before
   native loading. Keep a hard deadline and cleanup in the external owner.
3. Supply exact `NativeLibraries` paths/hashes to `OpenVinoRuntime::load`.
4. Verify a complete `VerifiedModelBundle`, then call `OpenVinoCpu::load`.
5. Generate with finite `GenerationOptions` and a `GenerationControl`.

Input, output and token budgets are bounded. CPU uses one inference thread and
one stream. Errors have stable codes; cancellation does not grant a new budget.
The native backend remains native code: Rust does not make model parsing or
in-process cancellation a sandbox. See [the interface](docs/interface-reference.md).

## Development and acceptance

Requires Rust 1.97 and Linux. Build/default tests use runtime linking, so no
native SDK, model download or GPU is required:

```sh
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test --locked -- --test-threads=1
```

[Development tests](docs/testing.md) and [explicit native acceptance](docs/native-acceptance.md)
are separate. The latter exercises Intel's real CPU runtime with a small,
synthetic model: deterministic generation, actual token usage, output limits,
pre-cancellation and reuse after cancellation. It does not establish useful
language quality, arbitrary models, GPU compatibility or long-term leak freedom.
No model weights or native runtime are bundled in the Rust package.

## Conversion status

Historical Python experiments are not the Rust API. Their Optimum Intel
exporter currently excludes the patched Transformers range; do not override
constraints or run it as a supported secure conversion path. The unresolved
conversion scope is tracked in [Issue #12](https://github.com/zixcel/devino/issues/12).
The Rust backend does not claim to implement PyTorch-to-OpenVINO conversion,
an HTTP model server or automatic GPU activation.

[Getting started](docs/getting-started.md) · [source contract](docs/oss-source-contract.md)
· [security](SECURITY.md) · [Apache-2.0](LICENSE) · [notices](NOTICE)
