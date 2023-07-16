
[01:46:09] test build::cargo::test::test_dedup_flags ... ok
[01:46:09] test server::test::server_message_get_method_name ... ok
[01:46:09] test server::test::server_message_to_str ... ok
[01:46:10] error[E0422]: cannot find struct, variant or union type `LibCfgTestStruct` in module `bin_lib_no_cfg_test`
[01:46:10]  --> test_data\bin_lib_no_cfg_test\src\main.rs:5:37
[01:46:10]   |
[01:46:10] 5 |     let test = bin_lib_no_cfg_test::LibCfgTestStruct { };
[01:46:10]   |                                     ^^^^^^^^^^^^^^^^ not found in `bin_lib_no_cfg_test`
[01:46:10]
[01:46:11] test test::test_bin_lib_project ... ok
[01:46:11] test test::test_borrow_error ... ok
[01:46:12] test test::test_completion ... ok
[01:46:12] test test::test_find_all_refs ... ok
[01:46:13] test test::test_find_all_refs_no_cfg_test ... ok
[01:46:13] test test::test_multiple_binaries ... ok
[01:46:13] test test::test_parse_error_on_malformed_input ... ok
[01:46:14] test test::test_reformat ... ok
[01:46:14] test test::test_reformat_with_range ... ok
[01:46:15] test test::test_simple_workspace ... ok
[01:46:19] test test::test_bin_lib_project_no_cfg_test ... FAILED
[01:46:19]
[01:46:19] failures:
[01:46:19]
[01:46:19] ---- test::test_bin_lib_project_no_cfg_test stdout ----
[01:46:19]  thread 'test::test_bin_lib_project_no_cfg_test' panicked at 'Hit timeout', src\tools\rls\src\test\harness.rs:142:12
