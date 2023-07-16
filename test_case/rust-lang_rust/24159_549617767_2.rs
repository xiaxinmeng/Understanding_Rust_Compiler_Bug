
error[E0277]: the trait bound `<F as Foo>::B: Bar<A>` is not satisfied
  --> src/main.rs:12:1
   |
3  |   trait Foo {
   |   --------- required by `Foo`
...
12 |   fn test<A, F: Foo<A=A>>(f: &F) {
   |   ^                             - help: consider further restricting the associated type: `where <F as Foo>::B: Bar<A>`
   |  _|
   | |
13 | |     test_bar(f.get_b());
14 | | }
   | |_^ the trait `Bar<A>` is not implemented for `<F as Foo>::B`
