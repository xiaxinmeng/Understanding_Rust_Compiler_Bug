plain
.................................................................................................... 1100/1185
...............iiiiii................................................................
failures:

---- src/primitive_docs.rs - prim_array (line 610) stdout ----
error: edition 2021 is unstable and only available with -Z unstable-options
Couldn't compile the test.

failures:
    src/primitive_docs.rs - prim_array (line 610)
    src/primitive_docs.rs - prim_array (line 610)

test result: FAILED. 1164 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 30.68s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:01:32
