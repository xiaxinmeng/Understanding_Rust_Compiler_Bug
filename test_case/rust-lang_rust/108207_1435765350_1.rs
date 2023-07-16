rust
error[[E0277]](https://doc.rust-lang.org/nightly/error_codes/E0277.html): can't compare `[u32; 4]` with `_` in const contexts
 --> src/lib.rs:5:12
  |
3 | #[derive_const(PartialEq)]
  |                --------- in this derive macro expansion
4 | #[derive(Debug, Eq)]
5 | struct Foo([u32; 4]);
  |            ^^^^^^^^ no implementation for `[u32; 4] == _`
  |
  = help: the trait `~const PartialEq<_>` is not implemented for `[u32; 4]`
note: the trait `PartialEq<_>` is implemented for `[u32; 4]`, but that implementation is not `const`
