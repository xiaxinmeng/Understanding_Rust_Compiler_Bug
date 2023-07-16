plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1010 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0498 (line 8866) stdout ----
error[E0463]: can't find crate for `foo`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8867:11
  |
3 | #![plugin(foo)] // ok!

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
