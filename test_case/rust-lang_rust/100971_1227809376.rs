plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1042 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0282 (line 5055) stdout ----
error[E0283]: type annotations needed
    --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5056:5
     |
3    | let x = "hello".chars().rev().collect();
     |     ^                         ------- type must be known at this point
     = note: cannot satisfy `_: FromIterator<char>`
note: required by a bound in `collect`
    --> /checkout/library/core/src/iter/traits/iterator.rs:1832:19
     |
     |
1832 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
help: consider specifying the type argument in the function call
     |
     |
3    | let x = "hello".chars().rev().collect::<B>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.
Some expected error codes were not found: ["E0282"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0282 (line 5055)

test result: FAILED. 1007 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out; finished in 9.00s
