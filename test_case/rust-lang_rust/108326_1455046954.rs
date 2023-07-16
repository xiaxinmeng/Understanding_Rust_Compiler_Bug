plain
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- process::tests::child_stdout_read_buf stdout ----
thread 'process::tests::child_stdout_read_buf' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }', library\std\src\process\tests.rs:134:29

failures:
    process::tests::child_stdout_read_buf


test result: FAILED. 893 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 13.07s

error: test failed, to rerun pass `-p std --lib`
Build completed unsuccessfully in 0:38:20
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
