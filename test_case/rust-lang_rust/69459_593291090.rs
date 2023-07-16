
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
  --> src/dimensions.rs:94:14
   |
94 |         Self(PhantomData)
   |              ^^^^^^^^^^^
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn)]` to the crate attributes to enable
