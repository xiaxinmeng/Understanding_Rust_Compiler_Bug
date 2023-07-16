
warning: `#[inline]` is ignored on struct fields, match arms and macro defs
 --> src/lib.rs:1:12
  |
1 | struct Foo(#[inline] ());
  |            ^^^^^^^^^
  |
  = note: `#[warn(unused_attributes)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: see issue #80564 <https://github.com/rust-lang/rust/issues/80564> for more information
