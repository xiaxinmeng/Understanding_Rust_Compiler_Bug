plain
2019-07-23T02:42:27.9477444Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T02:42:27.9477516Z 
2019-07-23T02:42:27.9477757Z   git checkout -b <new-branch-name>
2019-07-23T02:42:27.9477824Z 
2019-07-23T02:42:27.9478102Z HEAD is now at 47de56c8d Auto merge of #62805 - Xanewok:update-rls, r=Xanewok
2019-07-23T02:42:27.9628204Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T02:42:27.9631081Z ==============================================================================
2019-07-23T02:42:27.9631148Z Task         : Bash
2019-07-23T02:42:27.9631223Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T04:56:05.7018870Z test client_workspace_symbol_duplicates ... ok
2019-07-23T04:56:05.7019597Z 
2019-07-23T04:56:05.7019805Z failures:
2019-07-23T04:56:05.7019960Z 
2019-07-23T04:56:05.7020973Z ---- client_deglob stdout ----
2019-07-23T04:56:05.7022012Z Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob"), "rootUri": Null})})
2019-07-23T04:56:05.7023550Z Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-68401"), String("rls.deglobImports-68401")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
2019-07-23T04:56:05.7024114Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
2019-07-23T04:56:05.7024424Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("deglob cfg(test)"), "percentage": Null, "title": String("Building")})})
2019-07-23T04:56:05.7024881Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("deglob"), "percentage": Null, "title": String("Building")})})
2019-07-23T04:56:05.7025693Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
2019-07-23T04:56:05.7026027Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
2019-07-23T04:56:05.7026327Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
2019-07-23T04:56:05.7027391Z Sending: Object({"id": Number(100), "jsonrpc": String("2.0"), "method": String("textDocument/codeAction"), "params": Object({"context": Object({"diagnostics": Array([])}), "range": Object({"end": Object({"character": Number(0), "line": Number(2)}), "start": Object({"character": Number(0), "line": Number(2)})}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob/src/main.rs")})})})
2019-07-23T04:56:05.7028566Z Processing message: Object({"id": Number(100), "jsonrpc": String("2.0"), "result": Array([Object({"arguments": Array([Object({"location": Object({"range": Object({"end": Object({"character": Number(14), "line": Number(2)}), "start": Object({"character": Number(13), "line": Number(2)})}), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob/src/main.rs")}), "new_text": String("max")})]), "command": String("rls.deglobImports-68401"), "title": String("Deglob import")})])})
2019-07-23T04:56:05.7029658Z thread 'client_deglob' panicked at 'assertion failed: arguments[0]["new_text"].as_str() == Some("{Stdin, Stdout}")', src/tools/rls/tests/client.rs:1168:5
2019-07-23T04:56:05.7030282Z Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
2019-07-23T04:56:05.7030568Z Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
2019-07-23T04:56:05.7030765Z Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
2019-07-23T04:56:05.7030930Z 
---
2019-07-23T04:59:21.4391323Z Verifying status of edition-guide...
2019-07-23T04:59:21.4403775Z Verifying status of rls...
2019-07-23T04:59:21.4417336Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-07-23T04:59:21.4429108Z 
2019-07-23T04:59:21.4429723Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-07-23T04:59:21.4429801Z 
2019-07-23T04:59:21.4430201Z If you do intend to update 'rls', please check the error messages above and
2019-07-23T04:59:21.4430307Z commit another update.
2019-07-23T04:59:21.4430341Z 
2019-07-23T04:59:21.4430587Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-07-23T04:59:21.4430976Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-07-23T04:59:21.4431074Z proper steps.
2019-07-23T04:59:22.4340442Z ##[error]Bash exited with code '3'.
2019-07-23T04:59:22.4377885Z ##[section]Starting: Upload CPU usage statistics
2019-07-23T04:59:22.4382763Z ==============================================================================
2019-07-23T04:59:22.4382857Z Task         : Bash
2019-07-23T04:59:22.4382946Z Description  : Run a Bash script on macOS, Linux, or Windows
