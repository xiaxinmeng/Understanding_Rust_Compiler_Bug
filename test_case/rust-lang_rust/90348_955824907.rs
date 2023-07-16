plain
doc tests for: /checkout/src/doc/unstable-book/src/library-features/format-args-capture.md
doc tests for: /checkout/src/doc/unstable-book/src/library-features/global-asm.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/library-features/global-asm.md" "--test-args" ""

stdout ----

running 4 tests
---
---- /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 98) stdout ----
error[E0658]: const operands for inline assembly are unstable
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:102:31
  |
6 | global_asm!("movl ${}, %ecx", const 5, options(att_syntax));
  |
  = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
  = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:104:28
  |
8 | global_asm!("mov ecx, {}", const 5);
  |
  = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
  = help: add `#![feature(asm_const)]` to the crate attributes to enable

