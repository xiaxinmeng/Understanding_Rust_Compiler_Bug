plain
---- src/process.rs - process::Command::read_stdout (line 1064) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: InvalidData, message: "command exited with non-zero status" }', src/process.rs:12:41



failures:
failures:
    src/process.rs - process::Command::read_stdout (line 1064)

test result: FAILED. 1168 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out; finished in 17.46s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:19:35
