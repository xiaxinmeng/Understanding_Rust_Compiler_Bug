
error[E0277]: the trait bound `&i32: Foo` is not satisfied
 --> src/main.rs:8:25
  |
5 | fn foo<T: Foo>(x: T) { }
  |           --- required by this bound in `foo`
...
8 |     let x: &Fn(&i32) = &foo;
  |                         ^^^ the trait `Foo` is not implemented for `&i32`
