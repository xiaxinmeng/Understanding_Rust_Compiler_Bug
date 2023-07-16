
error[E0277]: the trait bound `B: Bar` is not satisfied
  --> src/main.rs:16:17
   |
16 | type FooFn<B> = impl Baz;
   |                 ^^^^^^^^ the trait `Bar` is not implemented for `B`
   |
note: required because of the requirements on the impl of `Baz` for `MyBaz<B>`
  --> src/main.rs:12:15
   |
12 | impl <B: Bar> Baz for MyBaz<B> {
   |               ^^^     ^^^^^^^^
help: consider restricting type parameter `B`
   |
16 | type FooFn<B: Bar> = impl Baz;
   |             +++++

For more information about this error, try `rustc --explain E0277`.
