
---- cmd_format_utf16_range stdout ----
Object(
    {
        "id": Number(
            66
        ),
        "jsonrpc": String(
            "2.0"
        ),
        "result": Array(
            [
                Object(
                    {
                        "newText": String(
                            "/* 😢😢😢😢😢😢😢 */\r\nfn main() {}\r\n"
                        ),
                        "range": Object(
                            {
                                "end": Object(
                                    {
                                        "character": Number(
                                            34
                                        ),
                                        "line": Number(
                                            0
                                        )
                                    }
                                ),
                                "start": Object(
                                    {
                                        "character": Number(
                                            0
                                        ),
                                        "line": Number(
                                            0
                                        )
                                    }
                                )
                            }
                        )
                    }
                )
            ]
        )
    }
)
thread 'cmd_format_utf16_range' panicked at 'assertion failed: `(left == right)`
  left: `["/* 😢😢😢😢😢😢😢 */\r\nfn main() {}\r\n"]`,
 right: `["/* 😢😢😢😢😢😢😢 */\nfn main() {}\n"]`', tools\rls\tests\tests.rs:1306:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
---rls-stdout---
Content-Length: 605
{"jsonrpc":"2.0","id":0,"result":{"capabilities":{"textDocumentSync":2,"hoverProvider":true,"completionProvider":{"resolveProvider":true,"triggerCharacters":[".",":"]},"definitionProvider":true,"implementationProvider":true,"referencesProvider":true,"documentHighlightProvider":true,"documentSymbolProvider":true,"workspaceSymbolProvider":true,"codeActionProvider":true,"codeLensProvider":{"resolveProvider":false},"documentFormattingProvider":true,"documentRangeFormattingProvider":false,"renameProvider":true,"executeCommandProvider":{"commands":["rls.applySuggestion-3340","rls.deglobImports-3340"]}}}}Content-Length: 92
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","title":"Building"}}Content-Length: 127
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"cmd_format_utf16_range","title":"Building"}}Content-Length: 137
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"cmd_format_utf16_range cfg(test)","title":"Building"}}Content-Length: 104
{"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_1","title":"Building"}}Content-Length: 92
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_0","title":"Indexing"}}Content-Length: 104
{"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_0","title":"Indexing"}}Content-Length: 180
{"jsonrpc":"2.0","id":66,"result":[{"range":{"start":{"line":0,"character":0},"end":{"line":0,"character":34}},"newText":"/* 😢😢😢😢😢😢😢 */\r\nfn main() {}\r\n"}]}
---------------
failures:
    cmd_format_utf16_range
test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
