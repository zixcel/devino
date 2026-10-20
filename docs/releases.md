# Versioned crate releases

Releases use this repository's immutable `v<version>` tag at accepted main.
The tag workflow admits successful main CI and builds an independent `.crate`,
then binds its contents and SHA256 to the source and manifest version.

## First publication

Crates.io requires initial publication before Trusted Publisher registration.
Until `REGISTRY_PUBLISH_ENABLED` is explicitly enabled, the tag workflow runs
`bootstrap-review` with the `crates-io-public` owner-review Environment. Approval
records source, tag, archive and digest as `bootstrap-approval.json`; it uploads
nothing to the registry and holds no Cargo publication credential in CI.

After that job succeeds, the authorized local operator must:

1. Download the prepared archive and approval receipt from the same successful
   tag run. Confirm their repository, source, tag, archive and SHA256 match.
2. Check out the exact tag source into a clean checkout, build/package using
   Rust 1.97 and public dependencies, and compare the reproduced package bytes.
3. Publish that same package with existing local Cargo authentication. Never
   copy the token into Git, workflow settings, arguments or a release receipt.
4. Verify the official registry checksum and anonymously downloaded bytes,
   then exercise the unpacked public crate as a separate consumer.

An owner-review receipt or successful workflow is not publication proof. If the
upload outcome is unclear, query the exact registry version before retrying.
Do not upload different bytes under the same version or move its tag.

## Subsequent OIDC releases

After initial upload, separately authorize and register the exact Trusted
Publisher binding (`zixcel/devino`, `publish-cargo.yml`, `crates-io-public`).
Only after verifying that registration may the publication variable be enabled.
The normal publish job preserves owner review, rechecks/reproduces prepared
bytes, obtains a short-lived OIDC token, publishes and verifies the official
checksum. There is no personal publication token in CI.

Reference: [crates.io Trusted Publishing](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/).
