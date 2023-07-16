text
error[E0277]: can't compare `std::string::String` with `<ExampleImpl as TypeConstructor<'a>>::BorrowedNamespace`
  --> src/lib.rs:23:5
   |
8  |     type Namespace: PartialEq + for<'a> PartialEq<<Self as TypeConstructor<'a>>::BorrowedNamespace>;
   |          --------- associated type defined here
...
22 | impl SelectorImpl for ExampleImpl {
   | --------------------------------- in this `impl` item
23 |     type Namespace = String;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `std::string::String == <ExampleImpl as TypeConstructor<'a>>::BorrowedNamespace`
   |
   = help: the trait `for<'a> std::cmp::PartialEq<<ExampleImpl as TypeConstructor<'a>>::BorrowedNamespace>` is not implemented for `std::string::String`

error: aborting due to previous error
