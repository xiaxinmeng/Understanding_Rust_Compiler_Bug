plain
[00:56:47]    Compiling serde v1.0.40
[00:56:47]    Compiling fnv v1.0.6
[00:56:47]    Compiling num-traits v0.2.2
[00:56:47]    Compiling crossbeam v0.3.2
[00:56:48]    Compiling rustfmt-nightly v0.8.2 (https://github.com/rust-lang-nursery/rustfmt?rev=5e5992517d3591e2708d4ca6b155dfcbdf3344b9#5e599251)
[00:56:51]    Compiling rls v0.129.0 (file:///checkout/src/tools/rls)
[00:56:51]    Compiling getopts v0.2.17
[00:56:51]    Compiling shell-escape v0.1.4
[00:56:53]    Compiling glob v0.2.11
---
[01:12:26] test actions::diagnostics::diagnostic_suggestion_test::suggest_macro_error_no_trait ... ok
[01:12:26] test build::cargo::test::test_dedup_flags ... ok
[01:12:26] test cmd::url_workaround_unc_canonicals ... ok
[01:12:26] test actions::diagnostics::diagnostic_suggestion_test::suggest_use_when_cannot_find_type ... ok
[01:12:26] test server::io::tests::read_message_fails_on_empty_input ... ok
[01:12:26] test config::clippy_preference_from_str ... ok
[01:12:26] test config::clippy_preference_from_str ... ok
[01:12:26] test server::io::tests::read_message_fails_when_content_type_is_invalid ... ok
[01:12:26] test server::io::tests::read_message_fails_when_content_is_not_valid_utf8 ... ok
[01:12:26] test server::io::tests::read_message_fails_when_header_line_is_invalid ... ok
[01:12:26] test server::io::tests::read_message_fails_when_input_contains_only_header ... ok
[01:12:26] test server::io::tests::read_message_fails_when_length_is_not_numeric ... ok
[01:12:26] test server::io::tests::read_message_fails_when_length_header_is_missing ... ok
[01:12:26] test server::io::tests::read_message_returns_message_from_input_with_unknown_headers ... ok
[01:12:26] test server::io::tests::read_message_fails_when_length_is_too_large_integer ... ok
[01:12:26] test server::message::test::deserialize_message_empty_params ... ok
[01:12:26] test server::io::tests::read_message_returns_message_from_input_with_multiple_headers ... ok
[01:12:26] test server::io::tests::read_message_returns_message_from_valid_lsr_input ... ok
[01:12:26] test server::message::test::raw_message_parses_valid_jsonrpc_request_with_numeric_id ... ok
[01:12:26] test server::message::test::serialize_message_no_params ... ok
[01:12:26] test server::message::test::serialize_message_empty_params ... ok
[01:12:26] test server::message::test::test_parse_as_notification ... ok
---
[01:12:38] 
[01:12:38] running 2 tests
[01:12:39] expect_messages:
[01:12:39]   results: [
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-29652\",\"rls.deglobImports-29652\"]}}}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"message\":\"member_lib\",\"title\":\"Building\"}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"message\":\"member_lib\",\"title\":\"Building\"}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"message\":\"member_bin\",\"title\":\"Building\"}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"message\":\"member_bin\",\"title\":\"Building\"}}",
---
[01:12:39] ]
[01:12:39] test cmd_test_simple_workspace ... ok
[01:12:39] expect_messages:
[01:12:39]   results: [
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-29651\",\"rls.deglobImports-29651\"]}}}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"message\":\"foo\",\"title\":\"Building\"}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"message\":\"foo\",\"title\":\"Building\"}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_1\",\"title\":\"Building\"}}",
[01:12:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_0\",\"title\":\"Indexing\"}}",
---
travis_time:end:1fb28960:start=1530744914321439780,finish=1530744914328862663,duration=7422883
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0567d7dc
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f8cb997
$ dmesg | grep -i kill
