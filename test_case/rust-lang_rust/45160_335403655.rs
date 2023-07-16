
[01:39:18] [m[m[32m[1m   Compiling[m git2-curl v0.7.0
[01:39:20] [m[m[32m[1m   Compiling[m cargo v0.23.0 (file:///checkout/src/tools/cargo)
[01:43:39] [m[m[32m[1m   Compiling[m rls v0.122.0 (file:///checkout/src/tools/rls)
[01:45:56] [m[m[32m[1m    Finished[m release [optimized] target(s) in 471.8 secs
[01:45:56] [m[m[32m[1m     Running[m build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/rls-6bb3c6a77c1f06b8
[01:45:56] 
[01:45:56] running 24 tests
[01:45:56] test actions::test::test_find_word_at_pos ... ok
[01:45:56] test build::cargo::test::test_dedup_flags ... ok
[01:45:56] test server::test_parse_as_notification ... ok
[01:45:57] test test::test_bin_lib_project ... ok
[01:45:57] test test::test_completion ... ok
[01:45:57] test test::test_find_all_refs ... FAILED
[01:45:57] test test::test_find_all_refs_no_cfg_test ... FAILED
[01:45:58] test test::test_goto_def ... FAILED
[01:45:59] test test::test_find_impls ... FAILED
[01:45:59] test test::test_highlight ... FAILED
[01:45:59] test test::test_hover ... FAILED
[01:46:56] test test::test_bin_lib_project_no_cfg_test has been running for over 60 seconds
[01:46:56] test test::test_borrow_error has been running for over 60 seconds
[01:46:58] test test::test_handle_utf8_directory has been running for over 60 seconds
[01:46:59] test test::test_infer_bin has been running for over 60 seconds
[01:51:16] test test::test_bin_lib_project_no_cfg_test ... FAILED
[01:51:16] test test::test_borrow_error ... FAILED
[01:51:18] test test::test_handle_utf8_directory ... FAILED
[01:51:19] test test::test_infer_bin ... FAILED
[01:51:19] test test::test_omit_init_build ... ok
[01:51:19] test test::test_parse_error_on_malformed_input ... ok
[01:51:20] test test::test_reformat ... ok
[01:51:20] test test::test_reformat_with_range ... ok
[01:51:20] test test::test_rename ... FAILED
[01:51:20] test test::test_workspace_symbol ... FAILED
[01:52:16] test test::test_infer_custom_bin has been running for over 60 seconds
[01:52:16] test test::test_infer_lib has been running for over 60 seconds
[01:52:18] test test::test_multiple_binaries has been running for over 60 seconds
[01:56:37] test test::test_infer_lib ... FAILED
[01:56:37] test test::test_infer_custom_bin ... FAILED
[01:56:38] test test::test_multiple_binaries ... FAILED
[01:56:38] 
[01:56:38] failures:
[01:56:38] 
[01:56:38] ---- test::test_find_all_refs stdout ----
[01:56:38] 	expect_messages:
[01:56:38]   results: [
[01:56:38]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}",
[01:56:38]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/beginBuild\",\"params\":null}",
[01:56:38]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}",
[01:56:38]     "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"
[01:56:38] ],
[01:56:38]   expected: [
[01:56:38]     ExpectedMessage {
[01:56:38]         id: Some(
[01:56:38]             0
[01:56:38]         ),
