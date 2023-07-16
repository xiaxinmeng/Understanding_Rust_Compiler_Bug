
failures:
---- test::test_deglob stdout ----
	expect_messages:
  results: [
    "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\",\"rls.deglobImports\"]}}}}",
    "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/beginBuild\",\"params\":null}",
    "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}",
    "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"
],
  expected: [
    ExpectedMessage {
        id: Some(
            0
        ),
        contains: [
            "rls.deglobImports"
        ]
    },
    ExpectedMessage {
        id: None,
        contains: [
            "beginBuild"
        ]
    },
    ExpectedMessage {
        id: None,
        contains: [
            "diagnosticsBegin"
        ]
    },
    ExpectedMessage {
        id: None,
        contains: [
            "diagnosticsEnd"
        ]
    }
]
expect_messages:
  results: [
    "{\"jsonrpc\":\"2.0\",\"id\":100,\"result\":[{\"title\":\"Deglob Import\",\"command\":\"rls.deglobImports\",\"arguments\":[{\"location\":{\"range\":{\"end\":{\"character\":14,\"line\":12},\"start\":{\"character\":13,\"line\":12}},\"uri\":\"file:///C:/projects/rust/src/tools/rls/test_data/deglob/src/main.rs\"},\"new_text\":\"{Stdin, Stdout}\"}]}]}"
],
  expected: [
    ExpectedMessage {
        id: Some(
            100
        ),
        contains: [
            "\"title\":\"Deglob Import\"",
            "\"command\":\"rls.deglobImports\"",
            "{\"location\":{\"range\":{\"end\":{\"character\":14,\"line\":12},\"start\":{\"character\":13,\"line\":12}},\"uri\":",
            "deglob/src/main.rs\"}",
            "\"new_text\":\"{Stdout, Stdin}\""
        ]
    }
]
thread 'test::test_deglob' panicked at 'Could not find `"new_text":"{Stdout, Stdin}"` in `{"jsonrpc":"2.0","id":100,"result":[{"title":"Deglob Import","command":"rls.deglobImports","arguments":[{"location":{"range":{"end":{"character":14,"line":12},"start":{"character":13,"line":12}},"uri":"file:///C:/projects/rust/src/tools/rls/test_data/deglob/src/main.rs"},"new_text":"{Stdin, Stdout}"}]}]}`', src\libcore\option.rs:874:4
failures:
    test::test_deglob
test result: FAILED. 27 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
error: test failed, to rerun pass '--bin rls'
