
error[E0277]: the trait bound `std::boxed::Box<dyn Foo>: Foo` is not satisfied
  --> src/main.rs:11:9
   |
1  | fn foo<F: Foo>(f: F) { panic!() }
   |    ---    --- required by this bound in `foo`
...
11 |     foo(x);
   |         ^ the trait `Foo` is not implemented for `std::boxed::Box<dyn Foo>`
