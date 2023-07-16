plain
........................................................................................ 176/202
..........................
failures:

---- spec::tests::aarch64_qnx_710 stdout ----
thread 'spec::tests::aarch64_qnx_710' panicked at 'assertion failed: `(left matches right)`
  left: `Unix(Yes)`,
 right: `LinkerFlavor::Gnu(..)`', compiler/rustc_target/src/spec/tests/tests_impl.rs:45:25


---- spec::tests::aarch64_qnx_700 stdout ----
thread 'spec::tests::aarch64_qnx_700' panicked at 'assertion failed: `(left matches right)`
  left: `Unix(Yes)`,
 right: `LinkerFlavor::Gnu(..)`', compiler/rustc_target/src/spec/tests/tests_impl.rs:45:25

failures:
failures:
error: test failed, to rerun pass `-p rustc_target --lib`
    spec::tests::aarch64_qnx_700
    spec::tests::aarch64_qnx_710
test result: FAILED. 200 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

Build completed unsuccessfully in 0:17:11
