
note: required because `Option<Foo>` needs to implement `Bar`
  --> src/main.rs:7:9
   |
7  | impl<T> Bar for Option<T> where T: Qux {}
   |         ^^^     ^^^^^^^^^
