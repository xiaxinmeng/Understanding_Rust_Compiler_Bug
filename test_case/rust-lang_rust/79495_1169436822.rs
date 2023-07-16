
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/main.rs:1:58
  |
1 | #![feature(arbitrary_enum_discriminant, core_intrinsics, generic_const_exprs)]
  |                                                          ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: enum discriminant depends on generics
 --> src/main.rs:9:15
  |
9 |     Some(T) = std::mem::size_of::<T>(),
  |               ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at /rust/compiler/rustc_middle/src/ty/adt.rs:459:26
