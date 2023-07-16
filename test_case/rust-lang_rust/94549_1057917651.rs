plain
....................................F.
failures:

---- thread::tests::test_is_finished stdout ----
thread 'thread::tests::test_is_finished' panicked at 'assertion failed: start.elapsed() < Duration::from_secs(2)', library/std/src/thread/tests.rs:74:9

failures:
    thread::tests::test_is_finished


test result: FAILED. 936 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.31s

error: test failed, to rerun pass '-p std --lib'
