
error[E0277]: the trait bound `<T as Foo>::Baz: Bar<U>` is not satisfied
  --> src/main.rs:9:1
   |
1  |   trait Foo {
   |   --------- required by `Foo`
...
9  |   fn x<T: Foo<Bar=U>, U>(t: &T) {
   |   ^                            - help: consider further restricting the associated type: `where <T as Foo>::Baz: Bar<U>`
   |  _|
   | |
10 | | }
   | |_^ the trait `Bar<U>` is not implemented for `<T as Foo>::Baz`
