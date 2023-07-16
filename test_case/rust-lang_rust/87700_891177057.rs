plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1012 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0530 (line 10212) stdout ----
error[E0432]: unresolved import `Enum`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10217:5
  |
7 | use Enum::*;
  |     ^^^^ maybe a missing crate `Enum`?
error[E0425]: cannot find function, tuple struct or tuple variant `WithField` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10218:7
  |
8 | match WithField(1) {
8 | match WithField(1) {
  |       ^^^^^^^^^ not found in this scope

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0432.
For more information about an error, try `rustc --explain E0425`.
Some expected error codes were not found: ["E0530"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0530 (line 10212)

test result: FAILED. 989 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out; finished in 8.40s
