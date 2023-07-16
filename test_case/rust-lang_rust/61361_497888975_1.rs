
error[E0282]: type annotations needed for `T` in `std::vec::Vec<T>`
  --> $DIR/vector-no-ann.rs:2:16
   |
LL |     let foo = Vec::new();
   |         ---   ^^^^^^^^ cannot infer type
   |         |
   |         help: give `foo` a type with the type parameter `T` specified: `foo: std::vec::Vec<T>`
