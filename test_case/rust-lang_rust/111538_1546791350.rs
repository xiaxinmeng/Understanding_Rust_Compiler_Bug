plain

failures:

---- config::tests::detect_src_and_out stdout ----
thread 'config::tests::detect_src_and_out' panicked at 'assertion failed: `(left == right)`
  left: `"/checkout/obj/build/tmp-dry-run"`,
 right: `"/checkout/obj/build"`', config/tests.rs:72:13
---- config::tests::download_ci_llvm stdout ----
error: test failed, to rerun pass `--lib`
downloading https://static.rust-lang.org/dist/2023-04-20/rust-std-beta-aarch64-apple-darwin.tar.xz
downloading https://static.rust-lang.org/dist/2023-04-20/rust-std-beta-aarch64-apple-darwin.tar.xz
thread 'config::tests::download_ci_llvm' panicked at 'failed to verify /checkout/obj/build/cache/2023-04-20/rust-std-beta-aarch64-apple-darwin.tar.xz', download.rs:557:17

failures:
    config::tests::detect_src_and_out
    config::tests::download_ci_llvm
