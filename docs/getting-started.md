# Using Devino

Use the Rust library from a Linux worker owned by your application. Start with
[the public interface](interface-reference.md) and
[the explicit CPU consumer example](../examples/native_cpu.rs).
The example spawns a protected worker; do not load native backends into an
interactive shell, editor, MCP host or other shared application process.

Run the default development checks first. Then select and verify a native
archive and an immutable, licensed model bundle. Follow
[native acceptance](native-acceptance.md); missing libraries are an error,
not a successful model test. Loading the OpenVINO entry library alone does
not prove its GenAI/tokenizer/plugin dependencies are present.

Do not use old system-install commands or add a local dependency patch to a
release package. The committed dependency must resolve from the public registry
for an independent consumer. Version declarations are not proof of publication.
