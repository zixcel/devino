# Devino source contract

Devino owns the Rust OpenVINO CPU backend. It does not own a model catalog,
artifact acquisition, application permissions, a runtime supervisor or a server.
Those responsibilities remain with their actual consumers and managed libraries.

| Manifest | Package | Boundary |
| --- | --- | --- |
| Cargo.toml | devino 0.1.0 | Protected-worker native loading and bounded CPU generation |

Source acceptance requires ordinary tests, all-target Clippy, formatting,
secret inspection and public-registry package verification. Local path patches
are development evidence only. A version declaration is not publication proof.
Native acceptance is separate and identifies the exact archive, dependencies,
model revision, license and tested device; see [native acceptance](native-acceptance.md).

Intel-maintained Rust bindings are used instead of hand-written unsafe FFI.
Only the external owner may admit native distribution and model bytes. Stable
errors must not disclose raw native diagnostics or user prompts. A native
failure must remain inside the owner's protected worker process. Default tests
must not exhaust host memory or change host crash collection settings.

Historical Python experiments remain outside the Rust API while their export
scope is resolved in Issue #12. Their incompatible Optimum/Transformers graph
is not solved by making Rust tests pass. Do not count a skipped model/device
case as successful, override dependency constraints or dismiss advisories to
obtain a green build.

Package publication, runtime adoption and application deployment are distinct
acceptance steps. No private central repository or sibling checkout is required
for the released package. Each release must bind its immutable version/source,
real package contents and registry checksum before being marked complete.
