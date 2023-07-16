
error[E0401]: can't use type parameters from outer function; try using a local type parameter instead
 --> src/main.rs:5:20
  |
5 |     impl Bar for S<T> {
  |                    ^ use of type variable from outer function

error[E0392]: parameter `T` is never used
 --> src/main.rs:4:14
  |
4 |     struct S<T>;
  |              ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`
