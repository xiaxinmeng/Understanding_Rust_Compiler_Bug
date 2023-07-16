plain
2019-07-21T22:16:35.1352221Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T22:16:35.1545633Z ##[command]git config gc.auto 0
2019-07-21T22:16:35.1614879Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T22:16:35.1669202Z ##[command]git config --get-all http.proxy
2019-07-21T22:16:35.1809024Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62805/merge:refs/remotes/pull/62805/merge
---
2019-07-21T22:17:08.2138267Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T22:17:08.2138298Z 
2019-07-21T22:17:08.2138516Z   git checkout -b <new-branch-name>
2019-07-21T22:17:08.2138543Z 
2019-07-21T22:17:08.2138587Z HEAD is now at 7ed1fff7a Merge c97dfb5f833a4fbf158393c45cf03393fb488fdc into 83dfe7b27cf2debecebedd3b038f9a1c2e05e051
2019-07-21T22:17:08.2281475Z ##[section]Finishing: Checkout
2019-07-21T22:17:08.2288629Z ##[section]Starting: Decide whether to run this job
2019-07-21T22:17:08.2291724Z Task         : Bash
2019-07-21T22:17:08.2291766Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-21T22:17:08.2291888Z Version      : 3.151.2
2019-07-21T22:17:08.2291927Z Author       : Microsoft Corporation
2019-07-21T22:17:08.2291927Z Author       : Microsoft Corporation
2019-07-21T22:17:08.2291971Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-21T22:17:08.2292035Z ==============================================================================
2019-07-21T22:17:08.3650569Z Generating script.
2019-07-21T22:17:08.3682978Z ========================== Starting Command Output ===========================
2019-07-21T22:17:08.3713675Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1b0d44a0-b908-47b3-af35-472b08f1ba85.sh
2019-07-21T22:17:08.4917801Z Executing the job since submodules are updated
2019-07-21T22:17:08.4994247Z ##[section]Finishing: Decide whether to run this job
2019-07-21T22:17:08.5004473Z ==============================================================================
2019-07-21T22:17:08.5004579Z Task         : Bash
2019-07-21T22:17:08.5004627Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-21T22:17:08.5004674Z Version      : 3.151.2
---
2019-07-22T00:26:46.4126352Z test client_workspace_symbol ... ok
2019-07-22T00:26:46.4126800Z 
2019-07-22T00:26:46.4127096Z failures:
2019-07-22T00:26:46.4127160Z 
2019-07-22T00:26:46.4128170Z ---- client_deglob stdout ----
2019-07-22T00:26:46.4129035Z Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob"), "rootUri": Null})})
2019-07-22T00:26:46.4130408Z Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-67608"), String("rls.deglobImports-67608")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
2019-07-22T00:26:46.4130723Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
2019-07-22T00:26:46.4130849Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("deglob cfg(test)"), "percentage": Null, "title": String("Building")})})
2019-07-22T00:26:46.4131070Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("deglob"), "percentage": Null, "title": String("Building")})})
2019-07-22T00:26:46.4131207Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
2019-07-22T00:26:46.4131426Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
2019-07-22T00:26:46.4131560Z Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
2019-07-22T00:26:46.4132376Z Sending: Object({"id": Number(100), "jsonrpc": String("2.0"), "method": String("textDocument/codeAction"), "params": Object({"context": Object({"diagnostics": Array([])}), "range": Object({"end": Object({"character": Number(0), "line": Number(2)}), "start": Object({"character": Number(0), "line": Number(2)})}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob/src/main.rs")})})})
2019-07-22T00:26:46.4133265Z Processing message: Object({"id": Number(100), "jsonrpc": String("2.0"), "result": Array([Object({"arguments": Array([Object({"location": Object({"range": Object({"end": Object({"character": Number(14), "line": Number(2)}), "start": Object({"character": Number(13), "line": Number(2)})}), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/deglob/src/main.rs")}), "new_text": String("max")})]), "command": String("rls.deglobImports-67608"), "title": String("Deglob import")})])})
2019-07-22T00:26:46.4133825Z thread 'client_deglob' panicked at 'assertion failed: arguments[0]["new_text"].as_str() == Some("{Stdin, Stdout}")', src/tools/rls/tests/client.rs:1168:5
2019-07-22T00:26:46.4134247Z Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
2019-07-22T00:26:46.4134319Z Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
2019-07-22T00:26:46.4134485Z Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
2019-07-22T00:26:46.4134659Z 
---
2019-07-22T00:29:59.5078710Z Verifying status of edition-guide...
2019-07-22T00:29:59.5090319Z Verifying status of rls...
2019-07-22T00:29:59.5103330Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-07-22T00:29:59.5113075Z 
2019-07-22T00:29:59.5122101Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-07-22T00:29:59.5122166Z 
2019-07-22T00:29:59.5122487Z If you do intend to update 'rls', please check the error messages above and
2019-07-22T00:29:59.5122575Z commit another update.
2019-07-22T00:29:59.5122604Z 
2019-07-22T00:29:59.5123033Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-07-22T00:29:59.5123289Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-07-22T00:29:59.5123351Z proper steps.
2019-07-22T00:30:00.4906530Z ##[error]Bash exited with code '3'.
2019-07-22T00:30:00.4942848Z ##[section]Starting: Checkout
2019-07-22T00:30:00.4944686Z ==============================================================================
2019-07-22T00:30:00.4944751Z Task         : Get sources
2019-07-22T00:30:00.4944793Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
