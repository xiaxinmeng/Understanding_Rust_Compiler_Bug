plain
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_3"), "message": Null, "percentage": Null, "title": String("Building")})})
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0425"), "message": String("cannot find function `fetch_u32` in module `library`\n\nnot found in `library`"), "range": Object({"end": Object({"character": Number(53), "line": Number(4)}), "start": Object({"character": Number(44), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/binary/src/main.rs")})})
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider prefixing with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected u32, found u64\n\nhelp: you can convert an `u64` to `u32` and panic if the converted value wouldn\'t fit: `super::fetch_u32().try_into().unwrap()`"), "range": Object({"end": Object({"character": Number(62), "line": Number(9)}), "start": Object({"character": Number(44), "line": Number(9)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/simple_workspace/library/src/lib.rs")})})
[01:55:22] thread 'client_changing_workspace_lib_retains_diagnostics' panicked at 'assertion failed: bin.diagnostics[0].message.contains("mismatched types\n\nexpected u32, found u64")', src/tools/rls/tests/client.rs:276:5
[01:55:22] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
[01:55:22] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
[01:55:22] Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
[01:55:22] Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
---
travis_time:end:09b5962c:start=1558439001863077490,finish=1558439001870208018,duration=7130528
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3568f8e9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0437be4e
travis_time:start:0437be4e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07452ebc
$ dmesg | grep -i kill
