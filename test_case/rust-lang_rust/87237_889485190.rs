plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1010 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16234) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0744 (line 15120) stdout ----
error[E0658]: `for` is not allowed in a `const`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:15124:5
6 | /     for i in 0..4 { // error!
7 | |         x += i;
8 | |     }
  | |_____^
---
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
Some expected error codes were not found: ["E0744"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0744 (line 15120)

test result: FAILED. 987 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out; finished in 8.42s
