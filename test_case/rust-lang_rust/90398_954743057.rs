plain
doc tests for: /checkout/src/doc/rustdoc/src/the-doc-attribute.md
doc tests for: /checkout/src/doc/rustdoc/src/unstable-features.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/unstable-features.md" "--test-args" ""

stdout ----

running 5 tests
---
test /checkout/src/doc/rustdoc/src/unstable-features.md - Unstable_features::Unstable_command_line_arguments::__allow_ (line 411) ... ok

failures:

---- /checkout/src/doc/rustdoc/src/unstable-features.md - Unstable_features::Document_keywords (line 147) stdout ----
error[E0658]: `#[doc(keyword)]` is experimental
  |
  |
3 | #[doc(keyword = "Keyword")]
  |
  = note: see issue #51315 <https://github.com/rust-lang/rust/issues/51315> for more information
  = note: see issue #51315 <https://github.com/rust-lang/rust/issues/51315> for more information
  = help: add `#![feature(doc_keyword)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
