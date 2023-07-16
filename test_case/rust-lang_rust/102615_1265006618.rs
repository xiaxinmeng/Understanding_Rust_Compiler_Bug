plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1044 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0790 (line 16926) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0010 (line 224) stdout ----
error[E0015]: cannot call non-const fn `Box::<i32>::new` in constants
  |
  |
5 | const CON : Box<i32> = Box::new(0);
  |
  = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
Some expected error codes were not found: ["E0010"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0010 (line 224)

test result: FAILED. 1009 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out; finished in 9.07s
