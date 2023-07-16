plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1002 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16104) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0542 (line 10591) stdout ----
error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be marked `const`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10599:1
   |
9  | #[rustc_const_stable(feature = "_stable_const_fn", since = "1.0.0")] // ok!
   | -------------------------------------------------------------------- attribute specified here
10 | fn _stable_const_fn() {}
   | ^^^^^^^^^^^^^^^^^^^^^ help: make the function or method const
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0545 (line 10672) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0545 (line 10672) stdout ----
error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be marked `const`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10680:1
   |
9  | #[rustc_const_unstable(feature = "_unstable_const_fn", issue = "1")] // ok!
   | -------------------------------------------------------------------- attribute specified here
10 | fn _unstable_const_fn() {}
   | ^^^^^^^^^^^^^^^^^^^^^^^ help: make the function or method const
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0547 (line 10744) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0547 (line 10744) stdout ----
error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be marked `const`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10755:1
9  | / #[rustc_const_unstable(
9  | / #[rustc_const_unstable(
10 | |     feature = "_unstable_const_fn",
11 | |     issue = "none"
12 | | )] // ok!
   | |__- attribute specified here
13 |   fn _unstable_const_fn() {}
   |   ^^^^^^^^^^^^^^^^^^^^^^^ help: make the function or method const
error: aborting due to previous error

Couldn't compile the test.

