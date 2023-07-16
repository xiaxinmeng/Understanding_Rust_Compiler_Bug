
[02:11:56] ---- cmd_changing_workspace_lib_retains_bin_diagnostics stdout ----
[02:11:56] thread 'cmd_changing_workspace_lib_retains_bin_diagnostics' panicked at 'assertion failed: `(left == right)`
[02:11:56]   left: `String("E0425")`,
[02:11:56]  right: `"unused_variables"`', tools\rls\tests\tests.rs:408:5
[02:11:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:11:56] ---rls-stdout---
[02:11:56] Content-Length: 605
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","id":0,"result":{"capabilities":{"textDocumentSync":2,"hoverProvider":true,"completionProvider":{"resolveProvider":true,"triggerCharacters":[".",":"]},"definitionProvider":true,"implementationProvider":true,"referencesProvider":true,"documentHighlightProvider":true,"documentSymbolProvider":true,"workspaceSymbolProvider":true,"codeActionProvider":true,"codeLensProvider":{"resolveProvider":false},"documentFormattingProvider":true,"documentRangeFormattingProvider":false,"renameProvider":true,"executeCommandProvider":{"commands":["rls.applySuggestion-2560","rls.deglobImports-2560"]}}}}Content-Length: 92
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","title":"Building"}}Content-Length: 112
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"library","title":"Building"}}Content-Length: 122
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"library cfg(test)","title":"Building"}}Content-Length: 111
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"binary","title":"Building"}}Content-Length: 121
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"binary cfg(test)","title":"Building"}}Content-Length: 104
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_1","title":"Building"}}Content-Length: 92
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_0","title":"Indexing"}}Content-Length: 733
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"unused_variables","message":"unused variable: `test_val`\n\nhelp: consider using `_test_val` instead: `_test_val`","range":{"end":{"character":36,"line":9},"start":{"character":28,"line":9}},"severity":2,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/rlsit/t0/simple_workspace/library/src/lib.rs"}}Content-Length: 493
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `val`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_val` instead: `_val`","range":{"end":{"character":27,"line":4},"start":{"character":24,"line":4}},"severity":2,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/rlsit/t0/simple_workspace/binary/src/main.rs"}}Content-Length: 104
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_0","title":"Indexing"}}Content-Length: 92
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","title":"Building"}}Content-Length: 112
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"library","title":"Building"}}Content-Length: 121
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"binary cfg(test)","title":"Building"}}Content-Length: 111
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"binary","title":"Building"}}Content-Length: 122
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_3","message":"library cfg(test)","title":"Building"}}Content-Length: 104
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_3","title":"Building"}}Content-Length: 92
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_2","title":"Indexing"}}Content-Length: 681
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"E0308","message":"mismatched types\n\nexpected u32, found u64","range":{"end":{"character":62,"line":9},"start":{"character":44,"line":9}},"severity":1,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/rlsit/t0/simple_workspace/library/src/lib.rs"}}Content-Length: 409
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"E0308","message":"mismatched types\n\nexpected u32, found u64","range":{"end":{"character":55,"line":4},"start":{"character":35,"line":4}},"severity":1,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/rlsit/t0/simple_workspace/binary/src/main.rs"}}Content-Length: 104
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_2","title":"Indexing"}}Content-Length: 92
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","title":"Building"}}Content-Length: 122
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"library cfg(test)","title":"Building"}}Content-Length: 112
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"library","title":"Building"}}Content-Length: 121
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"binary cfg(test)","title":"Building"}}Content-Length: 111
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_5","message":"binary","title":"Building"}}Content-Length: 104
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_5","title":"Building"}}Content-Length: 92
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_4","title":"Indexing"}}Content-Length: 733
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"unused_variables","message":"unused variable: `unused`\n\nnote: #[warn(unused_variables)] on by default\nhelp: consider using `_unused` instead: `_unused`","range":{"end":{"character":30,"line":2},"start":{"character":24,"line":2}},"severity":2,"source":"rustc"},{"code":"unused_variables","message":"unused variable: `test_val`\n\nhelp: consider using `_test_val` instead: `_test_val`","range":{"end":{"character":36,"line":9},"start":{"character":28,"line":9}},"severity":2,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/rlsit/t0/simple_workspace/library/src/lib.rs"}}Content-Length: 444
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"E0425","message":"cannot find function `fetch_u32` in module `library`\n\nnot found in `library`","range":{"end":{"character":53,"line":4},"start":{"character":44,"line":4}},"severity":1,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/rlsit/t0/simple_workspace/binary/src/main.rs"}}Content-Length: 104
[02:11:56] 
[02:11:56] {"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_4","title":"Indexing"}}
[02:11:56] ---------------
[02:11:56] 
[02:11:56] 
[02:11:56] failures:
[02:11:56]     cmd_changing_workspace_lib_retains_bin_diagnostics
[02:11:56] 
[02:11:56] test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
