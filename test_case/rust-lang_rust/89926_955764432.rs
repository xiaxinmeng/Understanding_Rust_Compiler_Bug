plain
thread '<unnamed>' panicked at 'Box<dyn Any>', library/std/src/thread/tests.rs:186:9
thread '<unnamed>' panicked at 'owned string', library/std/src/thread/tests.rs:170:9
thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:154:9
thread '<unnamed>' panicked at 'Box<dyn Any>', library/std/src/thread/tests.rs:205:33
...................................................................F.....F..................

---- time::tests::instant_duration_since_saturates stdout ----
---- time::tests::instant_duration_since_saturates stdout ----
thread 'time::tests::instant_duration_since_saturates' panicked at 'attempt to add with overflow', library/std/src/time.rs:294:25
---- time::tests::instant_saturating_duration_since_nopanic stdout ----
thread 'time::tests::instant_saturating_duration_since_nopanic' panicked at 'attempt to add with overflow', library/std/src/time.rs:294:25



failures:
    time::tests::instant_duration_since_saturates
    time::tests::instant_saturating_duration_since_nopanic

test result: FAILED. 890 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.32s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:24:40
