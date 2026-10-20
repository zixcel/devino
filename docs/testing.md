# Low-impact backend development tests

Use one build job and one test thread, with reduced debug output. Run the
common library and backend package sequentially. Resolve the declared public
crate versions; a local dependency patch is development evidence only, not
independent distribution acceptance.

```sh
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 cargo test --locked -- --test-threads=1
```

The default suite checks input bounds, deterministic deadline boundaries,
cancellation, small output limits and unavailable callback-state errors.
Deadline tests supply observations to an internal private clock boundary;
the production API still uses the monotonic system clock. No public fake-clock
control or automatic timeout renewal is exposed.

Do not generate a panic, fatal signal, allocation exhaustion or native model
run to test error handling. An explicit error at the same internal boundary
checks policy without triggering platform crash collection. The byte-limit
case uses a four-byte limit, not a large allocation.

Native inference, native crashes, long-duration leak profiling, physical OOM
and GPU compatibility are separate acceptance work. Passing the default suite
does not prove any of them. No native library is loaded, no model is downloaded,
and no key or host configuration is changed by the default suite.

Native worker setup now requires per-process dump prohibition in the common
backend context. The common package exercises real pidfd-only forced shutdown,
protected abnormal exit without a core dump, and physical writer recovery in
its default suite. These are not native-model inference tests. Its protection
handshake must complete before releasing the test fault; no WSL-wide collector
or host setting is changed. See the common package's testing contract.

An explicit real CPU acceptance example is provided separately. See
[native acceptance](native-acceptance.md) for exact tested distribution/model
identities and checks. It remains distinct from default policy tests.
