plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 926e7843eaa1794f15948395588eddedfb74a0d8 and 93d65a9f9af3bf228659bd036aee942c829643c5
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-69314"), String("rls.deglobImports-69314")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"message": String("failed to load manifest for dependency `member_a`"), "range": Object({"end": Object({"character": Number(0), "line": Number(9999)}), "start": Object({"character": Number(0), "line": Number(0)})}), "severity": Number(1)})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t26/invalid_member_toml/member_a/dodgy_member/Cargo.toml")})})
thread 'client_invalid_member_toml_manifest' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
---
.......... (50/59)
........  (59/59)


/checkout/src/test/rustdoc-gui/mobile.goml mobile... FAILED
[ERROR] (line 14) Error: Evaluation failed: expected `45` for property `offsetHeight` for selector `.mobile-topbar h2.location`, found `48`: for command `assert-property: (".mobile-topbar h2.location", {"offsetHeight": 45})`

/checkout/src/test/rustdoc-gui/type-declation-overflow.goml type-declation-overflow... FAILED
[ERROR] (line 35) Error: Evaluation failed: expected `504` for property `scrollWidth` for selector `.mobile-topbar .location`, found `493`: for command `assert-property: (".mobile-topbar .location", {"scrollWidth": "504"})`
Build completed unsuccessfully in 0:00:19
