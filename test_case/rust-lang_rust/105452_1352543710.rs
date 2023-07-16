plain
running 16 tests
...............F
failures:

---- tests::test_unstable_options_tracking_hash stdout ----
thread 'tests::test_unstable_options_tracking_hash' panicked at 'assertion failed: `(left != right)`
  left: `Some(true)`,
 right: `Some(true)`', compiler/rustc_interface/src/tests.rs:779:5


failures:
    tests::test_unstable_options_tracking_hash
    tests::test_unstable_options_tracking_hash

test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

error: test failed, to rerun pass `-p rustc_interface --lib`
