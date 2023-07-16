plain
2019-07-23T15:48:52.8096155Z test client_workspace_symbol_duplicates ... ok
2019-07-23T15:48:52.8096239Z 
2019-07-23T15:48:52.8096323Z failures:
2019-07-23T15:48:52.8096360Z 
2019-07-23T15:48:52.8097271Z ---- client_deglob stdout ----
2019-07-23T15:48:52.8097854Z Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob"), "rootUri": Null})})
2019-07-23T15:48:52.8099860Z Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-68404"), String("rls.deglobImports-68404")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
2019-07-23T15:48:52.8100429Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
2019-07-23T15:48:52.8100646Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("deglob cfg(test)"), "percentage": Null, "title": String("Building")})})
2019-07-23T15:48:52.8100855Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("deglob"), "percentage": Null, "title": String("Building")})})
2019-07-23T15:48:52.8101054Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
2019-07-23T15:48:52.8101248Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
2019-07-23T15:48:52.8101442Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
2019-07-23T15:48:52.8102425Z Sending: Object({"id": Number(100), "jsonrpc": String("2.0"), "method": String("textDocument/codeAction"), "params": Object({"context": Object({"diagnostics": Array([])}), "range": Object({"end": Object({"character": Number(0), "line": Number(2)}), "start": Object({"character": Number(0), "line": Number(2)})}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob/src/main.rs")})})})
2019-07-23T15:48:52.8103545Z Processing message: Object({"id": Number(100), "jsonrpc": String("2.0"), "result": Array([Object({"arguments": Array([Object({"location": Object({"range": Object({"end": Object({"character": Number(14), "line": Number(2)}), "start": Object({"character": Number(13), "line": Number(2)})}), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob/src/main.rs")}), "new_text": String("max")})]), "command": String("rls.deglobImports-68404"), "title": String("Deglob import")})])})
2019-07-23T15:48:52.8104051Z thread 'client_deglob' panicked at 'assertion failed: arguments[0]["new_text"].as_str() == Some("{Stdin, Stdout}")', src/tools/rls/tests/client.rs:1168:5
2019-07-23T15:48:52.8104303Z Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
2019-07-23T15:48:52.8104492Z Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
2019-07-23T15:48:52.8104605Z Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
2019-07-23T15:48:52.8104662Z 
---
2019-07-23T15:52:16.3862727Z Verifying status of edition-guide...
2019-07-23T15:52:16.3874435Z Verifying status of rls...
2019-07-23T15:52:16.3888226Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-07-23T15:52:16.3901517Z 
2019-07-23T15:52:16.3902337Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-07-23T15:52:16.3902816Z 
2019-07-23T15:52:16.3903533Z If you do intend to update 'rls', please check the error messages above and
2019-07-23T15:52:16.3903651Z commit another update.
2019-07-23T15:52:16.3903721Z 
2019-07-23T15:52:16.3903985Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-07-23T15:52:16.3904260Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-07-23T15:52:16.3904332Z proper steps.
2019-07-23T15:52:17.3955181Z ##[error]Bash exited with code '3'.
2019-07-23T15:52:17.3989239Z ##[section]Starting: Upload CPU usage statistics
2019-07-23T15:52:17.3996959Z ==============================================================================
2019-07-23T15:52:17.3997081Z Task         : Bash
2019-07-23T15:52:17.3997153Z Description  : Run a Bash script on macOS, Linux, or Windows
