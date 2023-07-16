plain
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:36:9
.................................................................................................... 400/875
.................................................................................................... 500/875
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `[104, 101, 108, 108, 111]`,
 right: `[0, 0, 0, 0, 0]`', library/std/src/os/./unix/net/tests.rs:49:9
...........................F........................................................................ 600/875
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:383:27
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: SendError { .. }', library/std/src/sync/mpsc/sync_tests.rs:371:24
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:250:19
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/tests.rs:322:28
---
thread '<unnamed>' panicked at 'Box<dyn Any>', library/std/src/thread/tests.rs:205:33
...........................................................................
failures:

---- os::imp::unix::net::tests::basic stdout ----
thread 'os::imp::unix::net::tests::basic' panicked at 'assertion failed: `(left == right)`
  left: `Some("/tmp/rust-783326236/sock")`,
 right: `Some("/tmp/rust-783326236/soc")`', library/std/src/os/./unix/net/tests.rs:54:5

failures:
    os::imp::unix::net::tests::basic


test result: FAILED. 874 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.31s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:03
