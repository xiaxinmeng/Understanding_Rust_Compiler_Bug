plain
---- src/primitive_docs.rs - prim_char (line 308) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/library/core/src/char/convert.rs:103:51



failures:
failures:
    src/primitive_docs.rs - prim_char (line 308)

test result: FAILED. 3693 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 56.97s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:33
