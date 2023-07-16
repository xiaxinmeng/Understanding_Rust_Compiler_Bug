
error[E0277]: `main` has invalid return type `i32`
 --> src/main.rs:6:9
  |
6 |     foo(1_i32);
  |     --- ^^^^^ `main` can only return types that implement `Termination`
  |     |
  |     required by a bound introduced by this call
  |
  = help: the trait `Termination` is not implemented for `i32`
note: required by a bound in `foo`
 --> src/main.rs:3:11
  |
3 | fn foo<T: Termination>(_: T) {}
  |           ^^^^^^^^^^^ required by this bound in `foo`
