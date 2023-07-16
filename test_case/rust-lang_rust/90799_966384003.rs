plain
thread '<unnamed>' panicked at 'Box<dyn Any>', library/std/src/thread/tests.rs:239:33
..............................................................................................
failures:

---- process::tests::test_capture_env_at_spawn stdout ----
thread 'process::tests::test_capture_env_at_spawn' panicked at 'Tried to setenv '"RUN_TEST_NEW_ENV2"' while process has multiple threads', library/std/src/sys/unix/os.rs:558:9

failures:
    process::tests::test_capture_env_at_spawn


test result: FAILED. 893 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.30s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:23:31
