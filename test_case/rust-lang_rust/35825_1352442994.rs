
error[E0277]: the size for values of type `(dyn Iterator<Item = ()> + 'static)` cannot be known at compilation time
 --> src/main.rs:3:17
  |
3 | fn do_something(iter: dyn Iterator<Item = SomeType>) {}
  |                 ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `(dyn Iterator<Item = ()> + 'static)`
  = help: unsized fn params are gated as an unstable feature
help: function arguments must have a statically known size, borrowed types always have a known size
  |
3 | fn do_something(iter: &dyn Iterator<Item = SomeType>) {}
  |                       +

error[E0746]: return type cannot have an unboxed trait object
 --> src/main.rs:5:27
  |
5 | fn get_some_iterator() -> dyn Iterator<Item = SomeType> {
  |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
  |
5 | fn get_some_iterator() -> T {
  |                           ~
help: use `impl Iterator<Item = SomeType>` as the return type if all return paths have the same type but you want to expose only the trait in the signature
  |
5 | fn get_some_iterator() -> impl Iterator<Item = SomeType> {
  |                           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
help: use a boxed trait object if all return paths implement trait `Iterator<Item = SomeType>`
  |
5 | fn get_some_iterator() -> Box<dyn Iterator<Item = SomeType>> {
  |                           ++++                             +
