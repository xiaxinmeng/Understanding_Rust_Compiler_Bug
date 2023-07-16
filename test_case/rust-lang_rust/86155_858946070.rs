plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 999 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0633::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 12869) stdout ----
error[E0557]: feature has been removed
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:12870:12
  |
2 | #![feature(unwind_attributes)]
  |
  = note: use the C-unwind ABI instead

error: cannot find attribute `unwind` in this scope
error: cannot find attribute `unwind` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:12872:3
  |
4 | #[unwind()] // error: expected one argument

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0557`.
