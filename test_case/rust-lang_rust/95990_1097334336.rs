plain
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- fs::tests::concurrent_recursive_mkdir stdout ----
thread 'fs::tests::concurrent_recursive_mkdir' panicked at 'failed to join thread: Invalid argument (os error 22)', library/std/src/sys/unix/thread.rs:252:13

failures:
    fs::tests::concurrent_recursive_mkdir


test result: FAILED. 943 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.45s

error: test failed, to rerun pass '-p std --lib'
