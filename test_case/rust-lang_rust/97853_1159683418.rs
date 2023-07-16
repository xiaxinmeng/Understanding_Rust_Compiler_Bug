plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5fb8a3926619a18a2e31627e4cdf05f16530b0eb and 21993a7f8c2356f50e58a0d9cafb95006d6a7166
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("infer_lib cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("infer_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("dead_code"), "message": String("struct `UnusedLib` is never constructed\n\nnote: `#[warn(dead_code)]` on by default"), "range": Object({"end": Object({"character": Number(16), "line": Number(0)}), "start": Object({"character": Number(7), "line": Number(0)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t19/infer_lib/src/lib.rs")})})
thread 'client_infer_lib' panicked at 'assertion failed: diag.diagnostics[0].message.contains(\"struct is never constructed: `UnusedLib`\")', src/tools/rls/tests/client.rs:1964:5
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
---
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("custom_bin cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("custom_bin"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("dead_code"), "message": String("struct `UnusedCustomBin` is never constructed\n\nnote: `#[warn(dead_code)]` on by default"), "range": Object({"end": Object({"character": Number(22), "line": Number(0)}), "start": Object({"character": Number(7), "line": Number(0)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t38/infer_custom_bin/src/custom_bin.rs")})})
thread 'client_test_infer_custom_bin' panicked at 'assertion failed: diag.diagnostics[0].message.contains(\"struct is never constructed: `UnusedCustomBin`\")', src/tools/rls/tests/client.rs:77:5
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
