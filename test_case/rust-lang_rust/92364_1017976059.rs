plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1029 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0644 (line 13477) stdout ----
error[E0308]: arguments to this function are incorrect
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13485:5
   |
9  |       fix(&|y| {
   |  _____^^^_-
10 | |         // Here, when `x` is called, the parameter `y` is equal to `x`.
11 | |     });
   | |_____- cyclic type of infinite size
   |
   = note: closures cannot capture themselves or take themselves as argument;
           this error may be the result of a recent compiler bug-fix,
           for more information
note: function defined here
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13478:4
   |
   |
2  | fn fix<F>(f: &F)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
Some expected error codes were not found: ["E0644"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0644 (line 13477)

test result: FAILED. 997 passed; 1 failed; 31 ignored; 0 measured; 0 filtered out; finished in 9.05s
