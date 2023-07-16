
Compiling playground v0.0.1 (/playground)
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the size for values of type `dyn Stream` cannot be known at compilation time
  --> src/lib.rs:14:5
   |
14 |     x.count();
   |     ^ ----- required by a bound introduced by this call
   |     |
   |     doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `dyn Stream`
note: required by a bound in `StreamExt::count`
  --> src/lib.rs:6:36
   |
6  |         fn count(self) where Self: Sized {}
   |                                    ^^^^^ required by this bound in `StreamExt::count`
