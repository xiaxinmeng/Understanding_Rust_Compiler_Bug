plain
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-5805"), String("rls.deglobImports-5805")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"message": String("failed to load manifest for dependency `member_a`"), "range": Object({"end": Object({"character": Number(0), "line": Number(9999)}), "start": Object({"character": Number(0), "line": Number(0)})}), "severity": Number(1)})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t26/invalid_member_toml/member_a/dodgy_member/Cargo.toml")})})
error: test failed, to rerun pass '--test client'
thread 'client_invalid_member_toml_manifest' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:267:28
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to test-fail during beta week.
{"cargo-miri":"test-fail","rls":"test-fail","book":"test-pass","embedded-book":"test-pass","nomicon":"test-pass","reference":"test-pass","rust-by-example":"test-pass","edition-guide":"test-pass","miri":"test-pass","rustbook":"test-fail"}Build completed unsuccessfully in 0:00:00
