
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src/lib.rs:3:12
  |
3 | fn foo(t: &Why) {}
  |            ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: only the last element of a tuple may have a dynamically sized type
