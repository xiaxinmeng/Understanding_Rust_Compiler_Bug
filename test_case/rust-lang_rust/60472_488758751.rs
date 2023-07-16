plain
travis_time:end:13d49f6b:start=1556810728806618253,finish=1556810804737333520,duration=75930715267
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `val`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_val`"), "range": Object({"end": Object({"character": Number(27), "line": Number(4)}), "start": Object({"character": Number(24), "line": Number(4)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `test_val`\n\nhelp: consider prefixing with an underscore: `_test_val`"), "range": Object({"end": Object({"character": Number(36), "line": Number(9)}), "start": Object({"character": Number(28), "line": Number(9)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:49:38] Sending: Object({"jsonrpc": String("2.0"), "method": String("textDocument/didChange"), "params": Object({"contentChanges": Array([Object({"range": Object({"end": Object({"character": Number(41), "line": Number(1)}), "start": Object({"character": Number(38), "line": Number(1)})}), "rangeLength": Number(3), "text": String("u64")})]), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs"), "version": Number(0)})})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("library"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_3"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0425"), "message": String("cannot find function `fetch_u32` in module `library`\n\nnot found in `library`"), "range": Object({"end": Object({"character": Number(53), "line": Number(4)}), "start": Object({"character": Number(44), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected u32, found u64\n\nhelp: you can convert an `u64` to `u32` or panic if it the converted value wouldn\'t fit: `super::fetch_u32().try_into().unwrap()`"), "range": Object({"end": Object({"character": Number(62), "line": Number(9)}), "start": Object({"character": Number(44), "line": Number(9)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:49:38] thread 'client_changing_workspace_lib_retains_diagnostics' panicked at 'assertion failed: bin.diagnostics[0].message.contains("mismatched types\n\nexpected u32, found u64")', src/tools/rls/tests/client.rs:276:5
[01:49:38] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
[01:49:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:49:38] Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
[01:49:38] Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
---
[01:56:36] Verifying status of edition-guide...
[01:56:36] Verifying status of rls...
[01:56:36] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:56:36] 
[01:56:36] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:56:36] 
[01:56:36] If you do intend to update 'rls', please check the error messages above and
[01:56:36] commit another update.
[01:56:36] 
[01:56:36] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:56:36] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:56:36] proper steps.
