plain
Testing stage2 error-index (x86_64-unknown-linux-gnu)
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1081 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0771::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler (line 16947) stdout ----
error[E0770]: the type of const parameters must not depend on other generic parameters
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16950:41
  |
5 | fn function_with_str<'a, const STRING: &'a str>() {} // error!
  |                                         ^^ the type must not depend on the parameter `'a`
  = note: lifetime parameters may not be used in the type of const parameters


warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
  |
2 | #![feature(adt_const_params)]
  |            ^^^^^^^^^^^^^^^^
  |
  |
  = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information
  = note: `#[warn(incomplete_features)]` on by default

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0770`.
Some expected error codes were not found: ["E0771"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0771::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler (line 16947)

test result: FAILED. 1031 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out; finished in 8.30s
