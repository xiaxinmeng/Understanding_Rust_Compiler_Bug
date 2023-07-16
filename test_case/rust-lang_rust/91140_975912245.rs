plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/infer-static-outlives-requirements.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/inline-const.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/inline-const.md" "--test-args" ""

stdout ----

running 3 tests
---
---- /checkout/src/doc/unstable-book/src/language-features/inline-const.md - The_tracking_issue_for_this_feature_is__ (line 32) stdout ----
error[E0658]: inline-const in pattern position is experimental
 --> /checkout/src/doc/unstable-book/src/language-features/inline-const.md:39:5
  |
9 |     const { 1 + 2 } => println!("Matched 1 + 2"),
  |
  = note: see issue #76001 <https://github.com/rust-lang/rust/issues/76001> for more information
  = help: add `#![feature(inline_const_pat)]` to the crate attributes to enable


error[E0658]: inline-const in pattern position is experimental
  --> /checkout/src/doc/unstable-book/src/language-features/inline-const.md:40:5
   |
10 |     const { one() } => println!("Matched const fn returning 1"),
   |
   = note: see issue #76001 <https://github.com/rust-lang/rust/issues/76001> for more information
   = help: add `#![feature(inline_const_pat)]` to the crate attributes to enable

