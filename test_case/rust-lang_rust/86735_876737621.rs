plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1002 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16106) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0665 (line 13436) stdout ----
error[E0658]: deriving `Default` on enums is experimental
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13437:10
3 | #[derive(Default)]
  |          ^^^^^^^
  |
  = note: see issue #86985 <https://github.com/rust-lang/rust/issues/86985> for more information
  = note: see issue #86985 <https://github.com/rust-lang/rust/issues/86985> for more information
  = help: add `#![feature(derive_default_enum)]` to the crate attributes to enable
  = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
Some expected error codes were not found: ["E0665"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0665 (line 13436)

test result: FAILED. 980 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out; finished in 6.85s
