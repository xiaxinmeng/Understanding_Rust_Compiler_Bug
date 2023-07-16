
error[E0252]: the name `A` is defined multiple times
 --> src/main.rs:8:14
  |
8 | use foo::{A, B as A};
  |           -  ^^^^^^ `A` reimported here
  |           |
  |           previous import of the type `A` here
  |
  = note: `A` must be defined only once in the type namespace of this module
help: You can use `as` to change the binding name of the import
  |
8 | use foo::{A, B as A as OtherA};
  |              ^^^^^^^^^^^^^^^^
