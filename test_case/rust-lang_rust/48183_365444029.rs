
01:44:09] failures:
[01:44:09] 
[01:44:09] ---- test::test_find_impls stdout ----
[01:44:09] 	expect_messages:
[01:44:09]   results: [
[01:44:09]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\",\"rls.deglobImports\"]}}}}",
[01:44:09]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/beginBuild\"}",
[01:44:09]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\"}",
[01:44:09]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\"}"
[01:44:09] ],
[01:44:09]   expected: [
[01:44:09]     ExpectedMessage {
[01:44:09]         id: Some(
[01:44:09]             0
[01:44:09]         ),
[01:44:09]         contains: [
[01:44:09]             "capabilities"
[01:44:09]         ]
[01:44:09]     },
[01:44:09]     ExpectedMessage {
[01:44:09]         id: None,
[01:44:09]         contains: [
[01:44:09]             "beginBuild"
[01:44:09]         ]
[01:44:09]     },
[01:44:09]     ExpectedMessage {
[01:44:09]         id: None,
[01:44:09]         contains: [
[01:44:09]             "diagnosticsBegin"
[01:44:09]         ]
[01:44:09]     },
[01:44:09]     ExpectedMessage {
[01:44:09]         id: None,
[01:44:09]         contains: [
[01:44:09]             "diagnosticsEnd"
[01:44:09]         ]
[01:44:09]     }
[01:44:09] ]
[01:44:09] expect_messages:
[01:44:09]   results: [
[01:44:09]     "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":[{\"uri\":\"file:///checkout/src/tools/rls/test_data/find_impls/src/main.rs\",\"range\":{\"start\":{\"line\":18,\"character\":15},\"end\":{\"line\":18,\"character\":18}}}]}"
[01:44:09] ],
[01:44:09]   expected: [
[01:44:09]     ExpectedMessage {
[01:44:09]         id: Some(
[01:44:09]             1
[01:44:09]         ),
[01:44:09]         contains: [
[01:44:09]             "\"range\":{\"start\":{\"line\":18,\"character\":15},\"end\":{\"line\":18,\"character\":18}}",
[01:44:09]             "\"range\":{\"start\":{\"line\":19,\"character\":12},\"end\":{\"line\":19,\"character\":15}}"
[01:44:09]         ]
[01:44:09]     }
[01:44:09] ]
[01:44:09] thread 'test::test_find_impls' panicked at 'Could not find `"range":{"start":{"line":19,"character":12},"end":{"line":19,"character":15}}` in `{"jsonrpc":"2.0","id":1,"result":[{"uri":"file:///checkout/src/tools/rls/test_data/find_impls/src/main.rs","range":{"start":{"line":18,"character":15},"end":{"line":18,"character":18}}}]}`', libcore/option.rs:917:5
[01:44:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:44:09] 
[01:44:09] 
[01:44:09] failures:
[01:44:09]     test::test_find_impls
[01:44:09] 
[01:44:09] test result: FAILED. 39 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
