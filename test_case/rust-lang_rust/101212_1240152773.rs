plain
running 16 tests
...............F
failures:

---- tests::test_unstable_options_tracking_hash stdout ----
thread 'tests::test_unstable_options_tracking_hash' panicked at 'assertion failed: `(left != right)`
  left: `false`,
 right: `false`', compiler/rustc_interface/src/tests.rs:736:5


failures:
    tests::test_unstable_options_tracking_hash
    tests::test_unstable_options_tracking_hash

test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass '-p rustc_interface --lib'
