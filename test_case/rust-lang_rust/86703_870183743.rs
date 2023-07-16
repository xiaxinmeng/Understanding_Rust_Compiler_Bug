rust
error: implementation of `Yokeable` is not general enough
  --> foo.rs:31:7
   |
31 |     y.project(move |yk, _| yk.as_bytes())
   |       ^^^^^^^ implementation of `Yokeable` is not general enough
   |
   = note: `&[u8]` must implement `Yokeable<'0>`, for any lifetime `'0`...
   = note: ...but `Yokeable<'_>` is actually implemented for the type `&'static [u8]`
