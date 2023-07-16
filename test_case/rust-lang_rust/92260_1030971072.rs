plain

failures:

---- builder::tests::defaults::build_cross_compile stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9

---- builder::tests::defaults::build_default stdout ----
---- builder::tests::defaults::build_default stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::defaults::build_stage_0 stdout ----
---- builder::tests::defaults::build_stage_0 stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::defaults::doc_default stdout ----
---- builder::tests::defaults::doc_default stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::build_all stdout ----
---- builder::tests::dist::build_all stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::build_with_empty_host stdout ----
---- builder::tests::dist::build_with_empty_host stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
error: test failed, to rerun pass '--lib'
---- builder::tests::dist::dist_baseline stdout ----
---- builder::tests::dist::dist_baseline stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_only_cross_host stdout ----
---- builder::tests::dist::dist_only_cross_host stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_empty_host stdout ----
---- builder::tests::dist::dist_with_empty_host stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_hosts stdout ----
---- builder::tests::dist::dist_with_hosts stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_targets stdout ----
---- builder::tests::dist::dist_with_targets stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::doc_ci stdout ----
---- builder::tests::dist::doc_ci stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::test_docs stdout ----
---- builder::tests::dist::test_docs stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::test_exclude stdout ----
---- builder::tests::dist::test_exclude stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9
---- builder::tests::dist::test_with_no_doc_stage0 stdout ----
---- builder::tests::dist::test_with_no_doc_stage0 stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30)', src/bootstrap/config.rs:683:9

failures:
    builder::tests::defaults::build_cross_compile
    builder::tests::defaults::build_default
