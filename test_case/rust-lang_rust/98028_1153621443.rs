plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1042 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0283 (line 5141) stdout ----
error: comparison operators cannot be chained
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5149:20
   |
10 | let bar: u32 = Into<u32>::into(foo) * 1u32;
   |                    ^   ^
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
10 | let bar: u32 = Into::<u32>::into(foo) * 1u32;

error: aborting due to previous error

Couldn't compile the test.
