
   Compiling playground v0.0.1 (/playground)
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src/lib.rs:1:6
  |
1 | fn f(a: str) {}
  |      ^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: function arguments must have a statically known size, borrowed types always have a known size
  |
1 | fn f(&a: str) {}
  |      ^

error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src/lib.rs:2:6
  |
2 | fn g(_: str) {}
  |      ^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: function arguments must have a statically known size, borrowed types always have a known size
  |
2 | fn g(_: &str) {}
