plain
test [ui] ui/panics/issue-47429-short-backtraces.rs#v0 ... ok
test [ui] ui/panics/issue-47429-short-backtraces.rs#legacy ... ok
test [ui] ui/panics/fmt-panic.rs ... ok
test [ui] ui/out-of-stack.rs ... ok
test [ui] ui/panics/location-detail-panic-no-column.rs ... ok
test [ui] ui/panics/panic-2021.rs ... ok
test [ui] ui/panics/location-detail-unwrap-no-file.rs ... ok
test [ui] ui/panic-runtime/abort.rs ... ok
test [ui] ui/panic-runtime/abort.rs ... ok
test [ui] ui/panics/location-detail-panic-no-line.rs ... ok
test [ui] ui/panics/location-detail-panic-no-file.rs ... ok
test [ui] ui/panics/panic-arg.rs ... ok
test [ui] ui/panics/panic-macro-any-wrapped.rs ... ok
test [ui] ui/panics/panic-macro-fmt.rs ... ok
test [ui] ui/panics/panic-macro-explicit.rs ... ok
---
test [ui] ui/panics/fmt-panic.rs ... ok
test [ui] ui/panics/issue-47429-short-backtraces.rs#v0 ... ok
test [ui] ui/panics/panic-2021.rs ... ok
test [ui] ui/panic-while-printing.rs ... ok
test [ui] ui/panics/location-detail-panic-no-column.rs ... ok
test [ui] ui/panics/location-detail-unwrap-no-file.rs ... ok
test [ui] ui/panics/location-detail-panic-no-file.rs ... ok
test [ui] ui/panics/location-detail-panic-no-line.rs ... ok
test [ui] ui/panics/main-panic.rs ... ok
test [ui] ui/panics/panic-macro-any-wrapped.rs ... ok
test [ui] ui/panics/panic-macro-any.rs ... ok
test [ui] ui/panics/panic-macro-fmt.rs ... ok
---
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- f32::tests::test_next_down stdout ----
thread 'f32::tests::test_next_down' panicked at 'assertion failed: `(left == right)`
 right: `2140493141`', library/std/src/f32/tests.rs:335:5


---- f32::tests::test_next_up stdout ----
thread 'f32::tests::test_next_up' panicked at 'assertion failed: `(left == right)`
 right: `2140493141`', library/std/src/f32/tests.rs:304:5

---- f64::tests::test_next_down stdout ----
---- f64::tests::test_next_down stdout ----
thread 'f64::tests::test_next_down' panicked at 'assertion failed: `(left == right)`
 right: `9219619037165300394`', library/std/src/f64/tests.rs:336:5


---- f64::tests::test_next_up stdout ----
thread 'f64::tests::test_next_up' panicked at 'assertion failed: `(left == right)`
 right: `9219619037165300394`', library/std/src/f64/tests.rs:305:5


failures:
failures:
    f32::tests::test_next_down
    f32::tests::test_next_up
    f64::tests::test_next_down
    f64::tests::test_next_up

test result: FAILED. 904 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.36s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i586-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:26:43
