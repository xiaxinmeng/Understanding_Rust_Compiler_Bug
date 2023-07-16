
error[E0277]: the size for values of type `(dyn Foo + 'static)` cannot be known at compilation time
  --> src/test/ui/feature-gates/feature-gate-unsized_fn_params.rs:19:8
   |
19 | fn foo(x: dyn Foo) {
   |        ^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Foo + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
