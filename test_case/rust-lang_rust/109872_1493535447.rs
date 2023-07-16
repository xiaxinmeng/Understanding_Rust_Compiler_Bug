plain
test setup::tests::check_matching_settings_hash ... ok

failures:

---- builder::tests::alias_and_path_for_library stdout ----
thread 'builder::tests::alias_and_path_for_library' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17

---- builder::tests::defaults::build_cross_compile stdout ----
---- builder::tests::defaults::build_cross_compile stdout ----
thread 'builder::tests::defaults::build_cross_compile' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::defaults::build_default stdout ----
---- builder::tests::defaults::build_default stdout ----
thread 'builder::tests::defaults::build_default' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::defaults::build_stage_0 stdout ----
---- builder::tests::defaults::build_stage_0 stdout ----
thread 'builder::tests::defaults::build_stage_0' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::defaults::doc_default stdout ----
---- builder::tests::defaults::doc_default stdout ----
thread 'builder::tests::defaults::doc_default' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::build_all stdout ----
---- builder::tests::dist::build_all stdout ----
thread 'builder::tests::dist::build_all' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::build_with_empty_host stdout ----
---- builder::tests::dist::build_with_empty_host stdout ----
thread 'builder::tests::dist::build_with_empty_host' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_baseline stdout ----
---- builder::tests::dist::dist_baseline stdout ----
thread 'builder::tests::dist::dist_baseline' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_only_cross_host stdout ----
---- builder::tests::dist::dist_only_cross_host stdout ----
thread 'builder::tests::dist::dist_only_cross_host' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_with_empty_host stdout ----
---- builder::tests::dist::dist_with_empty_host stdout ----
thread 'builder::tests::dist::dist_with_empty_host' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_with_hosts stdout ----
---- builder::tests::dist::dist_with_hosts stdout ----
thread 'builder::tests::dist::dist_with_hosts' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
thread 'builder::tests::dist::dist_with_same_targets_and_hosts' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_with_targets stdout ----
---- builder::tests::dist::dist_with_targets stdout ----
thread 'builder::tests::dist::dist_with_targets' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
thread 'builder::tests::dist::dist_with_targets_and_hosts' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::doc_ci stdout ----
---- builder::tests::dist::doc_ci stdout ----
thread 'builder::tests::dist::doc_ci' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::test_docs stdout ----
---- builder::tests::dist::test_docs stdout ----
thread 'builder::tests::dist::test_docs' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::dist::test_with_no_doc_stage0 stdout ----
---- builder::tests::dist::test_with_no_doc_stage0 stdout ----
thread 'builder::tests::dist::test_with_no_doc_stage0' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::test_exclude stdout ----
---- builder::tests::test_exclude stdout ----
thread 'builder::tests::test_exclude' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::test_exclude_kind stdout ----
---- builder::tests::test_exclude_kind stdout ----
thread 'builder::tests::test_exclude_kind' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- builder::tests::test_valid stdout ----
---- builder::tests::test_valid stdout ----
thread 'builder::tests::test_valid' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- config::tests::detect_src_and_out stdout ----
---- config::tests::detect_src_and_out stdout ----
thread 'config::tests::detect_src_and_out' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17
---- config::tests::download_ci_llvm stdout ----
---- config::tests::download_ci_llvm stdout ----
thread 'config::tests::download_ci_llvm' panicked at 'fs::remove_dir_all(&bin_root) failed with Access is denied. (os error 5)', download.rs:422:17

failures:
    builder::tests::alias_and_path_for_library
    builder::tests::defaults::build_cross_compile
---
test result: FAILED. 3 passed; 22 failed; 0 ignored; 0 measured; 0 filtered out; finished in 694.63ms

error: test failed, to rerun pass `--lib`
Build completed unsuccessfully in 1:10:32
make: *** [Makefile:68: ci-subset-1] Error 1
