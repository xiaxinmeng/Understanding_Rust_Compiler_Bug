plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1040 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0118 (line 2217) stdout ----
error[E0390]: cannot define inherent `impl` for primitive types
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2218:6
  |
3 | impl fn(u8) { // error: no nominal type found for inherent implementation
  |
  = help: consider using an extension trait instead

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0390`.
Some expected error codes were not found: ["E0118"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0118 (line 2217)

test result: FAILED. 1005 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out; finished in 8.37s
