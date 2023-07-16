
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> $DIR/unsized-fn-param.rs:1:20
   |
LL | fn foo1<T: ?Sized>(_: T) {
   |         --         ^ doesn't have a size known at compile-time
   |         |
   |         help: consider further restricting this bound: `T: std::marker::Sized +`
   |
   = help: the trait `std::marker::Sized` is not implemented for `T`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all function arguments must have a statically known size
   = help: unsized locals are gated as an unstable feature
