plain
    Finished release [optimized] target(s) in 2m 25s
     Running unittests (build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-6a3375838e0a0ea7)

running 929 tests
...............................................................................F.....F.............. 100/929
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/buffered/tests.rs:495:13
.................................................................................................... 300/929
.................................................................................................... 400/929
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:37:9
---
thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:189:9
.............................
failures:

---- error::tests::empty_lines_mid_message stdout ----
thread 'error::tests::empty_lines_mid_message' panicked at 'assertion failed: `(left == right)`
  left: `"line 1\n\nline 2\n\nCaused by:\n   0: line 1\n      \nline 2\n   1: line 1\n      \nline 2"`,
 right: `"line 1\n\nline 2\n\nCaused by:\n   0: line 1\n      \n      line 2\n   1: line 1\n      \n      line 2"`', library/std/src/error/tests.rs:415:5

---- error::tests::errors_that_start_with_newline_formats_correctly stdout ----
thread 'error::tests::errors_that_start_with_newline_formats_correctly' panicked at 'assertion failed: `(left == right)`
  left: `"\nThe message\n\n\nCaused by:\n   0: \nThe message\n      \n1: \nThe message\n      "`,
 right: `"\nThe message\n\n\nCaused by:\n   0: \n      The message\n      \n   1: \n      The message\n      "`', library/std/src/error/tests.rs:330:5

failures:
    error::tests::empty_lines_mid_message
    error::tests::errors_that_start_with_newline_formats_correctly
    error::tests::errors_that_start_with_newline_formats_correctly

test result: FAILED. 927 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.32s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:25:33
