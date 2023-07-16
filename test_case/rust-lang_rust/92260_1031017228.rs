plain
   Compiling globset v0.4.5
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 1m 02s
[src/bootstrap/config.rs:688] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Build completed successfully in 0:01:21
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
---
DirectMap2M:     6109184 kB
DirectMap1G:    54525952 kB
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.15s
[src/bootstrap/config.rs:688] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Skipping Set({test::src/tools/tidy}) because it is excluded
Suite(test::src/test/ui) not skipped for "bootstrap::test::Ui" -- not in [src/tools/tidy]
Suite(test::src/test/run-pass-valgrind) not skipped for "bootstrap::test::RunPassValgrind" -- not in [src/tools/tidy]
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
---

failures:

---- builder::tests::defaults::build_cross_compile stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9

---- builder::tests::defaults::build_default stdout ----
---- builder::tests::defaults::build_default stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::defaults::build_stage_0 stdout ----
---- builder::tests::defaults::build_stage_0 stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::defaults::doc_default stdout ----
---- builder::tests::defaults::doc_default stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::build_all stdout ----
---- builder::tests::dist::build_all stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::build_with_empty_host stdout ----
---- builder::tests::dist::build_with_empty_host stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_baseline stdout ----
---- builder::tests::dist::dist_baseline stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_only_cross_host stdout ----
---- builder::tests::dist::dist_only_cross_host stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_empty_host stdout ----
---- builder::tests::dist::dist_with_empty_host stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_hosts stdout ----
---- builder::tests::dist::dist_with_hosts stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_same_targets_and_hosts stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_targets stdout ----
---- builder::tests::dist::dist_with_targets stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::doc_ci stdout ----
---- builder::tests::dist::doc_ci stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::test_docs stdout ----
---- builder::tests::dist::test_docs stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::test_exclude stdout ----
---- builder::tests::dist::test_exclude stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9
---- builder::tests::dist::test_with_no_doc_stage0 stdout ----
---- builder::tests::dist::test_with_no_doc_stage0 stdout ----
thread 'main' panicked at 'fs::create_dir_all(&config.out) failed with Read-only file system (os error 30) ("failed to create build dir: build")', src/bootstrap/config.rs:683:9

failures:
    builder::tests::defaults::build_cross_compile
    builder::tests::defaults::build_default
