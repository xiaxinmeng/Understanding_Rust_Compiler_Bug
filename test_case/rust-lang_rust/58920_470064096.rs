plain
[01:56:26] 
[01:56:26] failures:
[01:56:26] 
[01:56:26] ---- client_changing_workspace_lib_retains_diagnostics stdout ----
[01:56:26] Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace"), "rootUri": Null})})
[01:56:26] Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-2438"), String("rls.deglobImports-2438")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "message": String("library"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "message": String("library cfg(test)"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "message": String("binary"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "message": String("binary cfg(test)"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_0"), "title": String("Indexing")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `val`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_val`"), "range": Object({"end": Object({"character": Number(27), "line": Number(4)}), "start": Object({"character": Number(24), "line": Number(4)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `test_val`\n\nhelp: consider prefixing with an underscore: `_test_val`"), "range": Object({"end": Object({"character": Number(36), "line": Number(9)}), "start": Object({"character": Number(28), "line": Number(9)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:56:26] Sending: Object({"jsonrpc": String("2.0"), "method": String("textDocument/didChange"), "params": Object({"contentChanges": Array([Object({"range": Object({"end": Object({"character": Number(41), "line": Number(1)}), "start": Object({"character": Number(38), "line": Number(1)})}), "rangeLength": Number(3), "text": String("u64")})]), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs"), "version": Number(0)})})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "title": String("Indexing")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_3"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_3"), "message": String("library cfg(test)"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_3"), "message": String("library"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_3"), "message": String("binary cfg(test)"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_3"), "message": String("binary"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_3"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_2"), "title": String("Indexing")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected u32, found u64"), "range": Object({"end": Object({"character": Number(55), "line": Number(4)}), "start": Object({"character": Number(35), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected u32, found u64"), "range": Object({"end": Object({"character": Number(62), "line": Number(9)}), "start": Object({"character": Number(44), "line": Number(9)})}), "severity": Number(1), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:56:26] Sending: Object({"jsonrpc": String("2.0"), "method": String("textDocument/didChange"), "params": Object({"contentChanges": Array([Object({"range": Object({"end": Object({"character": Number(41), "line": Number(1)}), "start": Object({"character": Number(38), "line": Number(1)})}), "rangeLength": Number(3), "text": String("u32")})]), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs"), "version": Number(1)})})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_2"), "title": String("Indexing")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_5"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_5"), "message": String("library"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_5"), "message": String("binary cfg(test)"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_5"), "message": String("binary"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_5"), "message": String("library cfg(test)"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_5"), "title": String("Building")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_4"), "title": String("Indexing")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0425"), "message": String("cannot find function `fetch_u32` in module `library`\n\nnot found in `library`"), "range": Object({"end": Object({"character": Number(53), "line": Number(4)}), "start": Object({"character": Number(44), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `test_val`\n\nhelp: consider prefixing with an underscore: `_test_val`"), "range": Object({"end": Object({"character": Number(36), "line": Number(9)}), "start": Object({"character": Number(28), "line": Number(9)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:56:26] thread 'client_changing_workspace_lib_retains_diagnostics' panicked at 'assertion failed: bin.diagnostics[0].message.contains("unused variable: `val`")', src/tools/rls/tests/client.rs:298:5
[01:56:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:56:26] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
[01:56:26] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_4"), "title": String("Indexing")})})
[01:56:26] Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
[01:56:26] Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
[01:56:26] 
[01:56:26] failures:
[01:56:26]     client_changing_workspace_lib_retains_diagnostics
[01:56:26] 
---
[02:03:41] Verifying status of rust-by-example...
[02:03:41] Verifying status of rls...
[02:03:41] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[02:03:41] 
[02:03:41] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[02:03:41] 
[02:03:41] If you do intend to update 'rls', please check the error messages above and
[02:03:41] commit another update.
[02:03:41] 
[02:03:41] If you do NOT intend to update 'rls', please ensure you did not accidentally
[02:03:41] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[02:03:41] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:1bc1048a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar  6 10:56:45 UTC 2019
---
travis_time:end:0082d1e8:start=1551869807274278785,finish=1551869807280862696,duration=6583911
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:28099280
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b3f2e45
travis_time:start:2b3f2e45
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:334c3aa0
$ dmesg | grep -i kill
