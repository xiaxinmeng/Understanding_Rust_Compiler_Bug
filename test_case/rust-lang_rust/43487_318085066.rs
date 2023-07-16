
     Running build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps\rls-63ddc6f60085b731.exe
running 14 tests
test build::cargo::test::test_dedup_flags ... ok
test server::test::server_message_get_method_name ... ok
test server::test::server_message_to_str ... ok
error[E0422]: cannot find struct, variant or union type `LibCfgTestStruct` in module `bin_lib_no_cfg_test`
 --> test_data\bin_lib_no_cfg_test\src\main.rs:5:37
  |
5 |     let test = bin_lib_no_cfg_test::LibCfgTestStruct { };
  |                                     ^^^^^^^^^^^^^^^^ not found in `bin_lib_no_cfg_test`
test test::test_bin_lib_project ... ok
test test::test_borrow_error ... ok
test test::test_completion ... ok
test test::test_find_all_refs ... ok
test test::test_find_all_refs_no_cfg_test ... ok
test test::test_multiple_binaries ... ok
test test::test_parse_error_on_malformed_input ... ok
test test::test_reformat ... ok
test test::test_reformat_with_range ... ok
test test::test_simple_workspace ... ok
test test::test_bin_lib_project_no_cfg_test ... FAILED
failures:
---- test::test_bin_lib_project_no_cfg_test stdout ----
	thread 'test::test_bin_lib_project_no_cfg_test' panicked at 'Hit timeout', src\tools\rls\src\test\harness.rs:142:12
failures:
    test::test_bin_lib_project_no_cfg_test
test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
error: test failed, to rerun pass '--bin rls'
command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "test" "-j" "2" "--target" "x86_64-pc-windows-msvc" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/rls/Cargo.toml"
expected success, got: exit code: 101
