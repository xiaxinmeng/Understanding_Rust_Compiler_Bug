plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1044 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0790 (line 16933) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0045 (line 1030) stdout ----
error[E0277]: functions with the "rust-call" ABI must take a single non-self tuple argument
  |
  |
6 |     fn foo(x: u8, ...); // error!
  |        ^^^ the trait `Tuple` is not implemented for `u8`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
Some expected error codes were not found: ["E0045"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0045 (line 1030)

test result: FAILED. 1009 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out; finished in 8.95s
