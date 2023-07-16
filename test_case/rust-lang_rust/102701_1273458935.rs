plain
........................................................................................ 176/202
..........................
failures:

---- spec::tests::aarch64_qnx_710 stdout ----
thread 'spec::tests::aarch64_qnx_710' panicked at 'assertion failed: `(left == right)`
  left: `Some(["-Vgcc_ntoaarch64le_cxx"])`,
 right: `None`', compiler/rustc_target/src/spec/tests/tests_impl.rs:85:17


---- spec::tests::aarch64_qnx_700 stdout ----
thread 'spec::tests::aarch64_qnx_700' panicked at 'assertion failed: `(left == right)`
  left: `Some(["-Vgcc_ntoaarch64le_cxx"])`,
 right: `None`', compiler/rustc_target/src/spec/tests/tests_impl.rs:85:17

failures:
    spec::tests::aarch64_qnx_700
    spec::tests::aarch64_qnx_710
    spec::tests::aarch64_qnx_710

test result: FAILED. 200 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p rustc_target --lib`
