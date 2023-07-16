plain
[01:29:31] test test_completion_suggests_arguments_in_statements ... ok
[01:29:31] 
[01:29:31] failures:
[01:29:31] 
[01:29:31] ---- cmd_changing_workspace_lib_retains_bin_diagnostics stdout ----
[01:29:31] thread 'cmd_changing_workspace_lib_retains_bin_diagnostics' panicked at 'assertion failed: `(left == right)`
[01:29:31]   left: `String("E0425")`,
[01:29:31]  right: `"unused_variables"`', tools/rls/tests/tests.rs:408:5
[01:29:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:29:31] ---rls-stdout---
[01:29:31] 
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","id":0,"result":{"capabilities":{"textDocumentSync":2,"hoverProvider":true,"completionProvider":{"resolveProvider":true,"triggerCharacters":[".",":"]},"definitionProvider":true,"implementationProvider":true,"referencesProvider":true,"documentHighlightProvider":true,"documentSymbolProvider":true,"workspaceSymbolProvider":true,"codeActionProvider":true,"codeLensProvider":{"resolveProvider":false},"documentFormattingProvider":true,"documentRangeFormattingProvider":false,"renameProvider":true,"executeCommandProvider":{"commands":["rls.applySuggestion-14841","rls.deglobImports-14841"]}}}}Content-Length: 92
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","title":"Building"}}Content-Length: 112
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"library","title":"Building"}}Content-Length: 122
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"library cfg(test)","title":"Building"}}Content-Length: 111
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"binary","title":"Building"}}Content-Length: 121
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"binary cfg(test)","title":"Building"}}Content-Length: 104
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_1","title":"Building"}}Content-Length: 92
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_0","title":"Indexing"}}Content-Length: 493
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `val`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_val` instead: `_val`","range":{"end":{"character":27,"line":4},"start":{"character":24,"line":4}},"severity":2,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/binary/src/main.rs"}}Content-Length: 733
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"unused_variables","message":"unused variable: `test_val`\n\nhelp: consider using `_test_val` instead: `_test_val`","range":{"end":{"character":36,"line":9},"start":{"character":28,"line":9}},"severity":2,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/library/src/lib.rs"}}Content-Length: 104
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_0","title":"Indexing"}}Content-Length: 92
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","title":"Building"}}Content-Length: 112
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"library","title":"Building"}}Content-Length: 111
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"binary","title":"Building"}}Content-Length: 121
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"binary cfg(test)","title":"Building"}}Content-Length: 122
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"library cfg(test)","title":"Building"}}Content-Length: 104
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_3","title":"Building"}}Content-Length: 92
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_2","title":"Indexing"}}Content-Length: 409
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"E0308","message":"mismatched types\n\nexpected u32, found u64","range":{"end":{"character":55,"line":4},"start":{"character":35,"line":4}},"severity":1,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/binary/src/main.rs"}}Content-Length: 681
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"E0308","message":"mismatched types\n\nexpected u32, found u64","range":{"end":{"character":62,"line":9},"start":{"character":44,"line":9}},"severity":1,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/library/src/lib.rs"}}Content-Length: 104
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_2","title":"Indexing"}}Content-Length: 92
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","title":"Building"}}Content-Length: 112
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"library","title":"Building"}}Content-Length: 111
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"binary","title":"Building"}}Content-Length: 121
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"binary cfg(test)","title":"Building"}}Content-Length: 122
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"library cfg(test)","title":"Building"}}Content-Length: 104
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_5","title":"Building"}}Content-Length: 92
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_4","title":"Indexing"}}Content-Length: 444
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"E0425","message":"cannot find function `fetch_u32` in module `library`\n\nnot found in `library`","range":{"end":{"character":53,"line":4},"start":{"character":44,"line":4}},"severity":1,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/binary/src/main.rs"}}Content-Length: 733
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"unused_variables","message":"unused variable: `test_val`\n\nhelp: consider using `_test_val` instead: `_test_val`","range":{"end":{"character":36,"line":9},"start":{"character":28,"line":9}},"severity":2,"source":"rustc"}],"uri":"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/simple_workspace/library/src/lib.rs"}}Content-Length: 104
[01:29:31] 
[01:29:31] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_4","title":"Indexing"}}
[01:29:31] 
[01:29:31] 
[01:29:31] failures:
[01:29:31]     cmd_changing_workspace_lib_retains_bin_diagnostics
---
travis_time:end:0304554c:start=1541547305772936013,finish=1541547305791140407,duration=18204394
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:257816a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c3c64a8
travis_time:start:0c3c64a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09ea56fd
$ dmesg | grep -i kill
