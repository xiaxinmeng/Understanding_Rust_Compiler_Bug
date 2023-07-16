\n\nIn this case, `foo` is defined, but is not a struct, so Rust can't use it as\none.\n"},"level":"error","spans":[{"file_name":"test_data\\bin_lib\\src\\main.rs","byte_start":121,"byte_end":137,"line_start":6,"line_end":6,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"    let test = bin_lib::LibCfgTestStruct { };","highlight_start":25,"highlight_end":41}],"label":"not found in `bin_lib`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}
[01:39:37] test test::test_bin_lib_project_no_cfg_test ... ok
[01:39:37] test test::test_borrow_error ... ok
[01:39:38] test test::test_completion ... ok
[01:39:38] test test::test_find_all_refs ... ok
[01:39:39] test test::test_find_all_refs_no_cfg_test ... ok
[01:39:39] test test::test_find_impls ... ok
[01:39:40] test test::test_goto_def ... ok
[01:39:41] test test::test_highlight ... ok
[01:39:41] test test::test_hover ... ok
[01:39:42] test test::test_infer_bin ... ok
[01:39:42] test test::test_infer_custom_bin ... ok
[01:39:43] test test::test_infer_lib ... ok
[01:39:43] test test::test_multiple_binaries ... ok
[01:40:37] test test::test_features has been running for over 60 seconds
[01:40:43] test test::test_no_default_features has been running for over 60 seconds
[01:44:57] test test::test_features ... FAILED
[01:44:57] test test::test_omit_init_build ... ok
[01:44:57] test test::test_parse_error_on_malformed_input ... ok
[01:44:58] test test::test_reformat ... ok
[01:44:58] test test::test_reformat_with_range ... ok
[01:44:59] test test::test_rename ... ok
[01:45:00] test test::test_workspace_symbol ... FAILED
[01:45:03] test test::test_no_default_features ... FAILED
[01:45:03] 
[01:45:03] failures:
[01:45:03] 
[01:45:03] ---- test::test_features stdout ----
[01:45:03] 	thread 'test::test_features' panicked at 'Hit timeout', src\tools\rls\src\test\harness.rs:189:12
[01:45:03] 
[01:45:03] ---- test::test_workspace_symbol stdout ----
[01:45:03] 	expect_messages:
[01:45:03]   results: [
[01:45:03]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}",
[01:45:03]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/beginBuild\",\"params\":null}",
[01:45:03]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}",
[01:45:03]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"
[01:45:03] ],
[01:45:03]   expected: [
[01:45:03]     ExpectedMessage {
[01:45:03]         id: Some(
[01:45:03]             0
[01:45:03]         ),
[01:45:03]         contains: [
[01:45:03]             "capabilities"
[01:45:03]         ]
[01:45:03]     },
[01:45:03]     ExpectedMessage {
[01:45:03]         id: None,
[01:45:03]         contains: [
[01:45:03]             "beginBuild"
[01:45:03]         ]
[01:45:03]     },
[01:45:03]     ExpectedMessage {
[01:45:03]         id: None,
[01:45:03]         contains: [
[01:45:03]             "diagnosticsBegin"
[01:45:03]         ]
[01:45:03]     },
[01:45:03]     ExpectedMessage {
[01:45:03]         id: None,
[01:45:03]         contains: [
[01:45:03]             "diagnosticsEnd"
[01:45:03]         ]
[01:45:03]     }
[01:45:03] ]
[01:45:03] expect_messages:
[01:45:03]   results: [
[01:45:03]     "{\"jsonrpc\":\"2.0\",\"id\":42,\"result\":[{\"name\":\"nemo\",\"kind\":12,\"location\":{\"uri\":\"file:///C:/projects/rust/src/tools/rls/test_data/workspace_symbol/src/main.rs\",\"range\":{\"start\":{\"line\":11,\"character\":11},\"end\":{\"line\":11,\"character\":15}}},\"containerName\":\"x\"},{\"name\":\"nemo\",\"kind\":2,\"location\":{\"uri\":\"file:///C:/projects/rust/src/tools/rls/test_data/workspace_symbol/src/foo.rs\",\"range\":{\"start\":{\"line\":0,\"character\":4},\"end\":{\"line\":0,\"character\":8}}},\"containerName\":\"foo\"}]}"
[01:45:03] ],
[01:45:03]   expected: [
[01:45:03]     ExpectedMessage {
[01:45:03]         id: Some(
[01:45:03]             42
[01:45:03]         ),
[01:45:03]         contains: [
[01:45:03]             "\"id\":42",
[01:45:03]             "main.rs",
[01:45:03]             "\"name\":\"nemo\"",
[01:45:03]             "\"kind\":12",
[01:45:03]             "\"range\":{\"start\":{\"line\":11,\"character\":11},\"end\":{\"line\":11,\"character\":15}}",
[01:45:03]             "\"containerName\":\"x\"",
[01:45:03]             "foo.rs",
[01:45:03]             "\"name\":\"nemo\"",
[01:45:03]             "\"kind\":5",
[01:45:03]             "\"range\":{\"start\":{\"line\":0,\"character\":4},\"end\":{\"line\":0,\"character\":8}}",
[01:45:03]             "\"containerName\":\"foo\""
[01:45:03]         ]
[01:45:03]     }
[01:45:03] ]
[01:45:03] thread 'test::test_workspace_symbol' panicked at 'Could not find `"kind":5` in `{"jsonrpc":"2.0","id":42,"result":[{"name":"nemo","kind":12,"location":{"uri":"file:///C:/projects/rust/src/tools/rls/test_data/workspace_symbol/src/main.rs","range":{"start":{"line":11,"character":11},"end":{"line":11,"character":15}}},"containerName":"x"},{"name":"nemo","kind":2,"location":{"uri":"file:///C:/projects/rust/src/tools/rls/test_data/workspace_symbol/src/foo.rs","range":{"start":{"line":0,"character":4},"end":{"line":0,"character":8}}},"containerName":"foo"}]}`', src\libcore\option.rs:839:4
[01:45:03] 
[01:45:03] ---- test::test_no_default_features stdout ----
[01:45:03] 	thread 'test::test_no_default_features' panicked at 'Hit timeout', src\tools\rls\src\test\harness.rs:189:12
[01:45:03] 
[01:45:03] 
[01:45:03] failures:
[01:45:03]     test::test_features
[01:45:03]     test::test_no_default_features
[01:45:03]     test::test_workspace_symbol
[01:45:03] 
[01:45:03] test result: FAILED. 23 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:03] 
[01:45:03] error: test failed, to rerun pass '--bin rls'
[01:45:03] 
[01:45:03] 
[01:45:03] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/rls\\Cargo.toml"
[01:45:03] expected success, got: exit code: 101
[01:45:03] 
[01:45:03] 
[01:45:03] You can disable the tool in `src/tools/toolstate.toml`
