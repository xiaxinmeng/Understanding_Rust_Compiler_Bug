plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1043 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0790 (line 16885) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0705 (line 14520) stdout ----
warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
  |
2 | #![feature(rust_2018_preview)]
  |            ^^^^^^^^^^^^^^^^^


warning[E0705]: the feature `test_2018_feature` is included in the Rust 2018 edition
  |
3 | #![feature(test_2018_feature)] // error: the feature
  |            ^^^^^^^^^^^^^^^^^

