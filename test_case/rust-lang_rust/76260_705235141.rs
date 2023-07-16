
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0778 (line 15556) stdout ----
error[E0658]: the `#[instruction_set]` attribute is an experimental feature
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:15557:1
  |
3 | #[instruction_set(arm::a32)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #74727 <https://github.com/rust-lang/rust/issues/74727> for more information
  = help: add `#![feature(isa_attribute)]` to the crate attributes to enable
