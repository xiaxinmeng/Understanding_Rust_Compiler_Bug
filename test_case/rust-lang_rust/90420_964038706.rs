plain
doc tests for: /checkout/src/doc/rustdoc/src/the-doc-attribute.md
doc tests for: /checkout/src/doc/rustdoc/src/unstable-features.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/unstable-features.md" "--test-args" ""

stdout ----

running 5 tests
---
test /checkout/src/doc/rustdoc/src/unstable-features.md - Unstable_features::Extensions_to_the_::__Recording_what_platforms_or_features_are_required_for_code_to_be_present (line 60) ... ok

failures:

---- /checkout/src/doc/rustdoc/src/unstable-features.md - Unstable_features::Document_keywords (line 151) stdout ----
error[E0658]: `#[doc(keyword)]` is meant for internal use only
  |
  |
6 | #[doc(keyword = "keyword")]
  |
  = note: see issue #90418 <https://github.com/rust-lang/rust/issues/90418> for more information
  = help: add `#![feature(rustdoc_internals)]` to the crate attributes to enable

