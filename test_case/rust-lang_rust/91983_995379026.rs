plain
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- f32::tests::test_next_down stdout ----
thread 'f32::tests::test_next_down' panicked at 'assertion failed: `(left == right)`
 right: `2140493141`', library/std/src/f32/tests.rs:358:9

---- f32::tests::test_next_up stdout ----
---- f32::tests::test_next_up stdout ----
thread 'f32::tests::test_next_up' panicked at 'assertion failed: `(left == right)`
 right: `2140493141`', library/std/src/f32/tests.rs:321:9

---- f64::tests::test_next_down stdout ----
---- f64::tests::test_next_down stdout ----
thread 'f64::tests::test_next_down' panicked at 'assertion failed: `(left == right)`
 right: `9219619037165300394`', library/std/src/f64/tests.rs:347:9

---- f64::tests::test_next_up stdout ----
---- f64::tests::test_next_up stdout ----
thread 'f64::tests::test_next_up' panicked at 'assertion failed: `(left == right)`
 right: `9219619037165300394`', library/std/src/f64/tests.rs:310:9


failures:
failures:
    f32::tests::test_next_down
    f32::tests::test_next_up
    f64::tests::test_next_down
    f64::tests::test_next_up

test result: FAILED. 915 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.38s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i686-unknown-linux-musl" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:35:25
