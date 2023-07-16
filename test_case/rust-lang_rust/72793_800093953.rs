`
error[E0658]: `impl Trait` in type aliases is unstable
 --> bad.rs:3:25
  |
3 | type FilteredIter<'a> = impl Iterator<Item = &'a i32> + 'a;
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
  = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable
  