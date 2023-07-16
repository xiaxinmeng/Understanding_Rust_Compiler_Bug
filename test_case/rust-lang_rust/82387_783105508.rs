
error[E0277]: the trait bound `Result<Foo, ()>: IsFoo` is not satisfied
  --> src/main.rs:11:9
   |
7  | fn foo(x: impl CanFoo) {}
   |                ------ required by this bound in `foo`
...
11 |     foo(bar());
   |         ^^^^^ the trait `IsFoo` is not implemented for `Result<Foo, ()>`
   |
   = note: required because of the requirements on the impl of `CanFoo` for `Result<Foo, ()>`
help: `Result<Foo, ()>` doesn't implement `IsFoo`, but `Foo` does
   |
11 |     foo(bar().unwrap());
   |              ^^^^^^^^^
