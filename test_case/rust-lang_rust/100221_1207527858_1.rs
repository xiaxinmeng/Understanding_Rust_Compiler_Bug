
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the size for values of type `[u8]` cannot be known at compilation time
 --> src/lib.rs:4:33
  |
4 |     fn needs_sized(&self) where Self: Sized {}
  |                                 ^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: within `Bar`, the trait `Sized` is not implemented for `[u8]`
note: required because it appears within the type `Bar`
 --> src/lib.rs:1:12
  |
1 | pub struct Bar([u8]);
  |            ^^^
  = help: see issue #48214
