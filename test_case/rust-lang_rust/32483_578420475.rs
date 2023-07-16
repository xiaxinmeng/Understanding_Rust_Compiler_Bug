
error[E0277]: the trait bound `<T as Foo>::TypeB: Bar<u8>` is not satisfied
  --> src/lib.rs:17:5
   |
5  |     type TypeB: Bar<Self::TypeA>;
   |          ----- associated type defined here
...
15 | impl<T> Foo for T {
   | ------------------ help: consider further restricting the associated type: `where <T as Foo>::TypeB: Bar<u8>`
   | |
   | in this `impl` item
16 |     type TypeA = u8;
17 |     default type TypeB = ImplsBar;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bar<u8>` is not implemented for `<T as Foo>::TypeB`
