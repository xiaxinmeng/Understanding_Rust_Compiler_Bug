plain
[01:23:27] test actions::hover::test::test_extract_docs_empty_line_before_decl ... ok
[01:23:27] test actions::hover::test::test_extract_docs_module_docs ... ok
[01:23:27] test actions::hover::test::test_extract_docs_module_docs_no_copyright ... ok
[01:23:27] test actions::diagnostics::diagnostic_suggestion_test::suggest_mut_when_not_mut ... ok
[01:23:27] test actions::hover::test::test_extract_docs_module_docs_with_attribute ... ok
[01:23:27] test actions::hover::test::test_process_docs_bash_block ... ok
[01:23:27] test actions::hover::test::test_format_method ... ok
[01:23:27] test actions::hover::test::test_process_docs_rust_blocks ... ok
[01:23:27] test actions::hover::test::test_tooltip ... ignored
[01:23:27] test actions::hover::test::test_format_object ... ok
[01:23:27] test actions::hover::test::test_process_docs_racer_returns_extra_slashes ... ok
[01:23:27] test actions::requests::test::test_sort_deglob_str ... ok
[01:23:27] test actions::hover::test::test_noindent ... ok
[01:23:27] test cmd::url_workaround_unc_canonicals ... ok
[01:23:27] test server::io::tests::read_message_fails_on_empty_input ... ok
---
[01:23:36]     }
[01:23:36] ]
[01:23:36] expect_messages_unordered:
[01:23:36]   results: [
[01:23:36]     "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"diagnostics\":[{\"code\":\"unused_variables\",\"message\":\"unused variable: `val`\\n\\nnote: #[warn(unused_variables)] on by default\\nhelp: consider using `_val` instead: `_val`\",\"range\":{\"end\":{\"character\":27,\"line\":4},\"start\":{\"character\":24,\"line\":4}},\"severity\":2,\"source\":\"rustc\"}],\"uri\":\"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs\"}}",
[01:23:36]     "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"diagnostics\":[{\"code\":\"unused_variables\",\"message\":\"unused variable: `unused`\\n\\nnote: #[warn(unused_variables)] on by default\\nhelp: consider using `_unused` instead: `_unused`\",\"range\":{\"end\":{\"character\":30,\"line\":2},\"start\":{\"character\":24,\"line\":2}},\"severity\":2,\"source\":\"rustc\"},{\"code\":\"unused_variables\",\"message\":\"unused variable: `test_val`\\n\\nhelp: consider using `_test_val` instead: `_test_val`\",\"range\":{\"end\":{\"character\":36,\"line\":9},\"start\":{\"character\":28,\"line\":9}},\"severity\":2,\"source\":\"rustc\"}],\"uri\":\"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs\"}}"
[01:23:36]   expected: [
[01:23:36]     ExpectedMessage {
[01:23:36]         id: None,
[01:23:36]         contains: [
---
[01:23:37]     }
[01:23:37] ]
[01:23:37] expect_messages_unordered:
[01:23:37]   results: [
[01:23:37]     "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"diagnostics\":[{\"code\":\"E0308\",\"message\":\"mismatched types\\n\\nexpected u32, found u64\",\"range\":{\"end\":{\"character\":55,\"line\":4},\"start\":{\"character\":35,\"line\":4}},\"severity\":1,\"source\":\"rustc\"}],\"uri\":\"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs\"}}",
[01:23:37]     "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"diagnostics\":[{\"code\":\"E0308\",\"message\":\"mismatched types\\n\\nexpected u32, found u64\",\"range\":{\"end\":{\"character\":62,\"line\":9},\"start\":{\"character\":44,\"line\":9}},\"severity\":1,\"source\":\"rustc\"},{\"code\":\"unused_variables\",\"message\":\"unused variable: `unused`\\n\\nnote: #[warn(unused_variables)] on by default\\nhelp: consider using `_unused` instead: `_unused`\",\"range\":{\"end\":{\"character\":30,\"line\":2},\"start\":{\"character\":24,\"line\":2}},\"severity\":2,\"source\":\"rustc\"}],\"uri\":\"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs\"}}"
[01:23:37]   expected: [
[01:23:37]     ExpectedMessage {
[01:23:37]         id: None,
[01:23:37]         contains: [
[01:23:37]         contains: [
[01:23:37]             "publishDiagnostics",
[01:23:37]             "library/src/lib.rs",
[01:23:37]             "unused variable: `unused`",
[01:23:37]             "expected u32, found u64"
[01:23:37]     },
[01:23:37]     ExpectedMessage {
[01:23:37]         id: None,
[01:23:37]         contains: [
[01:23:37]         contains: [
[01:23:37]             "publishDiagnostics",
[01:23:37]             "binary/src/main.rs",
[01:23:37]             "expected u32, found u64"
[01:23:37]     }
[01:23:37] ]
[01:23:37] expect_messages:
[01:23:37]   results: [
---
[01:23:39]     }
[01:23:39] ]
[01:23:39] expect_messages_unordered:
[01:23:39]   results: [
[01:23:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"diagnostics\":[{\"code\":\"unused_variables\",\"message\":\"unused variable: `val`\\n\\nnote: #[warn(unused_variables)] on by default\\nhelp: consider using `_val` instead: `_val`\",\"range\":{\"end\":{\"character\":27,\"line\":4},\"start\":{\"character\":24,\"line\":4}},\"severity\":2,\"source\":\"rustc\"}],\"uri\":\"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs\"}}",
[01:23:39]     "{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"diagnostics\":[{\"code\":\"unused_variables\",\"message\":\"unused variable: `unused`\\n\\nnote: #[warn(unused_variables)] on by default\\nhelp: consider using `_unused` instead: `_unused`\",\"range\":{\"end\":{\"character\":30,\"line\":2},\"start\":{\"character\":24,\"line\":2}},\"severity\":2,\"source\":\"rustc\"},{\"code\":\"unused_variables\",\"message\":\"unused variable: `test_val`\\n\\nhelp: consider using `_test_val` instead: `_test_val`\",\"range\":{\"end\":{\"character\":36,\"line\":9},\"start\":{\"character\":28,\"line\":9}},\"severity\":2,\"source\":\"rustc\"}],\"uri\":\"file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs\"}}"
[01:23:39]   expected: [
[01:23:39]     ExpectedMessage {
[01:23:39]         id: None,
[01:23:39]         contains: [
---
[01:29:01] test config::file_lines::test::test_range_contains ... ok
[01:29:01] test config::file_lines::test::test_range_merge ... ok
[01:29:01] test config::file_lines::test::test_range_intersects ... ok
[01:29:01] test config::file_lines::test::test_range_adjacent_to ... ok
[01:29:01] test config::options::test_newline_style_auto_apply ... ok
[01:29:01] test config::license::test::test_parse_license_template ... ok
[01:29:01] test config::options::test_newline_style_auto_detect ... ok
[01:29:01] test config::test::test_config_used_to_toml ... ok
[01:29:01] test config::test::test_print_docs_exclude_unstable ... ok
[01:29:01] test config::test::test_print_docs_include_unstable ... ok
[01:29:01] test config::test::test_was_set ... ok
---
travis_time:end:35932cee:start=1533277998863333022,finish=1533277998873060496,duration=9727474
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d97bf14
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0087ec14
travis_time:start:0087ec14
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:297fe164
$ dmesg | grep -i kill
