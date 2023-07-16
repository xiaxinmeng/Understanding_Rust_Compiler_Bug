
error[E0277]: the size for values of type `(dyn A + 'static)` cannot be known at compilation time
  --> $DIR/closure-return-type-must-be-sized.rs:54:5
   |
LL |     a::foo::<fn() -> dyn A>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `(dyn A + 'static)`
