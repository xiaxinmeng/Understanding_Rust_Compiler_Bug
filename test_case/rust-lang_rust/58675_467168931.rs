plain
[01:55:38] 
[01:55:38] failures:
[01:55:38] 
[01:55:38] ---- client_find_definitions stdout ----
[01:55:38] Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({}), "initializationOptions": Object({"settings": Object({"rust": Object({"racer_completion": Bool(false)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/simple_workspace"), "rootUri": Null})})
[01:55:38] Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-2324"), String("rls.deglobImports-2324")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
[01:55:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "title": String("Building")})})
[01:55:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "message": String("bar"), "title": String("Building")})})
[01:55:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_1"), "message": String("bar cfg(test)"), "title": String("Building")})})
[01:55:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "title": String("Building")})})
[01:55:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"id": String("progress_0"), "title": String("Indexing")})})
[01:55:38] Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "title": String("Indexing")})})
[01:55:38] Processing message: Object({"id": Number(100), "jsonrpc": String("2.0"), "result": Array([])})
[01:55:38] Sending: Object({"id": Number(101), "jsonrpc": String("2.0"), "method": String("textDocument/definition"), "params": Object({"position": Object({"character": Number(1), "line": Number(1)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/simple_workspace/src/main.rs")})})})
[01:55:38] Processing message: Object({"id": Number(101), "jsonrpc": String("2.0"), "result": Array([])})
[01:55:38] Sending: Object({"id": Number(102), "jsonrpc": String("2.0"), "method": String("textDocument/definition"), "params": Object({"position": Object({"character": Number(2), "line": Number(1)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/simple_workspace/src/main.rs")})})})
---
[01:55:38] Sending: Object({"id": Number(1202), "jsonrpc": String("2.0"), "method": String("textDocument/definition"), "params": Object({"position": Object({"character": Number(2), "line": Number(12)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/simple_workspace/src/main.rs")})})})
[01:55:38] Processing message: Object({"id": Number(1202), "jsonrpc": String("2.0"), "result": Array([])})
[01:55:38] Sending: Object({"id": Number(1203), "jsonrpc": String("2.0"), "method": String("textDocument/definition"), "params": Object({"position": Object({"character": Number(3), "line": Number(12)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/simple_workspace/src/main.rs")})})})
[01:55:38] Processing message: Object({"id": Number(1203), "jsonrpc": String("2.0"), "result": Array([])})
[01:55:38] Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
[01:55:38] Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
[01:55:38] Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
[01:55:38] thread 'client_find_definitions' panicked at 'Got different amount of completions than expected: 24 vs. 25: [
[01:55:38]         1,
[01:55:38]         16,
[01:55:38]         [
[01:55:38]             Range {
---
travis_time:end:0ff0a2a8:start=1551126201421048682,finish=1551126201453381257,duration=32332575
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09ba9f9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:049f56a0
travis_time:start:049f56a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02f27fba
$ dmesg | grep -i kill
