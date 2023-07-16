
   Compiling playground v0.0.1 (/playground)
error[E0107]: missing generics for trait `Foo`
 --> src/main.rs:8:10
  |
8 |     <dyn Foo>::hi(123); // bare_trait_objects
  |          ^^^ expected 1 generic argument
  |
note: trait defined here, with 1 generic parameter: `T`
 --> src/main.rs:1:7
  |
1 | trait Foo<T> {}
  |       ^^^ -
help: add missing generic argument
  |
8 |     <dyn Foo<T>>::hi(123); // bare_trait_objects
  |          ~~~~~~

For more information about this error, try `rustc --explain E0107`.
error: could not compile `playground` due to previous error
