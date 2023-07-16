
[01:12:17] ---- test::test_find_all_refs stdout ----
...
[01:12:17] thread 'test::test_find_all_refs' panicked at 'Could not find `{"start":{"line":9,"character":7},"end":{"line":9,"character":10}}` in `{"jsonrpc":"2.0","id":42,"result":[]}`', /checkout/src/libcore/option.rs:891:4
[01:12:17] 
[01:12:17] ---- test::test_find_all_refs_no_cfg_test stdout ----
...
[01:12:17] thread 'test::test_find_all_refs_no_cfg_test' panicked at 'Could not find `{"start":{"line":9,"character":7},"end":{"line":9,"character":10}}` in `{"jsonrpc":"2.0","id":42,"result":[]}`', /checkout/src/libcore/option.rs:891:4
...
[01:12:17] failures:
[01:12:17]     test::test_borrow_error
[01:12:17]     test::test_features
[01:12:17]     test::test_find_all_refs
[01:12:17]     test::test_find_all_refs_no_cfg_test
[01:12:17]     test::test_find_impls
[01:12:17]     test::test_goto_def
[01:12:17]     test::test_highlight
[01:12:17]     test::test_hover
[01:12:17]     test::test_infer_bin
[01:12:17]     test::test_infer_custom_bin
[01:12:17]     test::test_infer_lib
[01:12:17]     test::test_multiple_binaries
[01:12:17]     test::test_no_default_features
[01:12:17]     test::test_parse_error_on_malformed_input
[01:12:17]     test::test_rename
[01:12:17]     test::test_workspace_symbol
[01:12:17] 
[01:12:17] test result: FAILED. 11 passed; 16 failed; 0 ignored; 0 measured; 0 filtered out
