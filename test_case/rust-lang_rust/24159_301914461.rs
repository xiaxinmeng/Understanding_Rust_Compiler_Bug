
error[E0277]: the trait bound `<F as Foo>::B: Bar<A>` is not satisfied
  --> test.rs:14:1
   |
14 | / fn test<A, F: Foo<A=A>>(f: &F) {
15 | |     test_bar(f.get_b());
16 | | }
   | |_^ the trait `Bar<A>` is not implemented for `<F as Foo>::B`
   |
   = help: consider adding a `where <F as Foo>::B: Bar<A>` bound
   = note: required by `Foo`

error[E0277]: the trait bound `<F as Foo>::C: Car<A>` is not satisfied
  --> test.rs:14:1
   |
14 | / fn test<A, F: Foo<A=A>>(f: &F) {
15 | |     test_bar(f.get_b());
16 | | }
   | |_^ the trait `Car<A>` is not implemented for `<F as Foo>::C`
   |
   = help: consider adding a `where <F as Foo>::C: Car<A>` bound
   = note: required by `Foo`

error: aborting due to 2 previous errors
