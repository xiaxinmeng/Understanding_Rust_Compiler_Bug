plain
[01:54:25] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_5"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:54:25] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_5"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:54:25] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_4"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:54:25] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `test_val`\n\nhelp: consider prefixing with an underscore: `_test_val`"), "range": Object({"end": Object({"character": Number(36), "line": Number(9)}), "start": Object({"character": Number(28), "line": Number(9)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:54:25] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0425"), "message": String("cannot find function `fetch_u32` in module `library`\n\nnot found in `library`"), "range": Object({"end": Object({"character": Number(53), "line": Number(4)}), "start": Object({"character": Number(44), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:54:25] thread 'client_changing_workspace_lib_retains_diagnostics' panicked at 'assertion failed: bin.diagnostics[0].message.contains("unused variable: `val`")', src/tools/rls/tests/client.rs:299:5
[01:54:25] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
[01:54:25] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_4"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:54:25] Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
[01:54:25] Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
---
travis_time:end:08346918:start=1561770409931538979,finish=1561770409941061113,duration=9522134
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0408b3a9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02e94e9e
travis_time:start:02e94e9e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:25b73dc4
$ dmesg | grep -i kill
