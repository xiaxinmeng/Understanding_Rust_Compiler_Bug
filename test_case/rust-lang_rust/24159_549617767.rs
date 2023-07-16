
error[E0277]: the trait bound `<F as Foo>::B: Bar<A>` is not satisfied
  --> src/lib.rs:14:1
   |
4  |   trait Foo {
   |   --------- required by `Foo`
...
14 |   fn test<A, F: Foo<A=A>>(f: &F) {
   |   ^                             - help: consider further restricting the associated type: `where <F as Foo>::B: Bar<A>`
   |  _|
   | |
15 | |     test_bar(f.get_b());
16 | | }
   | |_^ the trait `Bar<A>` is not implemented for `<F as Foo>::B`

error[E0277]: the trait bound `<F as Foo>::C: Car<A>` is not satisfied
  --> src/lib.rs:14:1
   |
4  |   trait Foo {
   |   --------- required by `Foo`
...
14 |   fn test<A, F: Foo<A=A>>(f: &F) {
   |   ^                             - help: consider further restricting the associated type: `where <F as Foo>::C: Car<A>`
   |  _|
   | |
15 | |     test_bar(f.get_b());
16 | | }
   | |_^ the trait `Car<A>` is not implemented for `<F as Foo>::C`
