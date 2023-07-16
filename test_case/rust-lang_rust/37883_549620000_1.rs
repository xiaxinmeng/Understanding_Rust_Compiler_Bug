
error[E0277]: the trait bound `<A as Arthur>::Wibble: Foo<()>` is not satisfied
  --> src/main.rs:10:5
   |
4  | pub fn assert_is_foo<A: Foo<()>>() { }
   |        -------------    ------- required by this bound in `assert_is_foo`
...
7  | pub fn test<A: Arthur>() {
   |                         - help: consider further restricting the associated type: `where <A as Arthur>::Wibble: Foo<()>`
...
10 |     assert_is_foo::<A::Wibble>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo<()>` is not implemented for `<A as Arthur>::Wibble`
