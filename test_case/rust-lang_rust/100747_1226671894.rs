plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1045 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0311 (line 5550) stdout ----
error[E0424]: expected value, found module `self`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5551:13
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_obj_build_x86_64_unknown_linux_gnu_test_error_index_md_5550_0() {
  |                                         ------------------------------------------------------------------------------------- this function can't have a `self` parameter
3 | let u_ref = self.borrow_mut();
  |             ^^^^ `self` value is a keyword only available in methods with a `self` parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0424`.
Couldn't compile the test.
