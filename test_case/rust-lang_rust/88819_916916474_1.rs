
error[E0277]: the trait bound `i32: Foo` is not satisfied
  --> foo.rs:14:15
   |
LL |     some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied
   |               ^^^^ the trait `Foo` is not implemented for `i32`
   |
note: required by a bound in `some_func`
  --> foo.rs:7:17
   |
LL | fn some_func<T: Foo>(foo: T) {
   |                 ^^^ required by this bound in `some_func`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
