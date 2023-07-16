
error[E0277]: the size for values of type `impl ?Sized` cannot be known at compilation time
 --> src/lib.rs:1:8
  |
1 | fn foo(x: impl ?Sized) {
  |        ^  ----------- help: consider further restricting this bound: `impl ?Sized + std::marker::Sized`
  |        |
  |        doesn't have a size known at compile-time

error[E0277]: the size for values of type `X` cannot be known at compilation time
 --> src/lib.rs:5:20
  |
5 | fn foo2<X: ?Sized>(x: X) {
  |         --         ^ doesn't have a size known at compile-time
  |         |
  |         help: consider further restricting this bound: `X: std::marker::Sized +`
