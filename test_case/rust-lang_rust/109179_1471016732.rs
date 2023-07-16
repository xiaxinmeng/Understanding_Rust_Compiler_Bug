plain
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:356:28
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: SendError { .. }', library/std/src/sync/mpsc/sync_tests.rs:381:24
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:393:27
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:260:19
Fatal glibc error: rseq registration failed
error: test failed, to rerun pass `-p std --lib`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/std-2ca099a455fabafa --quiet` (signal: 6, SIGABRT: process abort signal)
...............................Build completed unsuccessfully in 0:01:12
