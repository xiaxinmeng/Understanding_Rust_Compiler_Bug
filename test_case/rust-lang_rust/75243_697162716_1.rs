
warning: a method with this name may be added to the standard library in the future
  --> src/main.rs:10:15
   |
10 |     [4].map(|x| x);
   |         ^^^
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this method is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `ArrayExt::map(...)` to keep using the current method
   = help: add `#![feature(array_map)]` to the crate attributes to enable `array::<impl [T; N]>::map`
