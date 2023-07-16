plain
running 5 tests
F....
failures:

---- symbols::tests::test_symbols stdout ----
thread 'symbols::tests::test_symbols' panicked at 'called `Result::unwrap()` on an `Err` value: Error("LexError")', compiler/rustc_macros/src/symbols/tests.rs:13:48


failures:
    symbols::tests::test_symbols
    symbols::tests::test_symbols

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_macros --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_macros" "--" "--quiet"


Build completed unsuccessfully in 0:23:43
