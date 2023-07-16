
error[E0282]: type annotations needed for `std::vec::Vec<T>`
  --> $DIR/vector-no-ann.rs:2:16
   |
LL |     let foo = Vec::new();
   |         ---   ^^^^^^^^ cannot infer type for `T`
   |         |
   |         consider giving `foo` the type `std::vec::Vec<T>` with the type parameter `T` specified
