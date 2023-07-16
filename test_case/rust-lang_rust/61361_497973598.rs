
error[E0282]: type annotations needed for `std::vec::Vec<T>`
  --> $DIR/vector-no-ann.rs:2:16
   |
LL |     let foo = Vec::new();
   |         ---   ^^^^^^^^ cannot infer type for `T`
   |         |
   |         consider giving `foo` the explicit type `std::vec::Vec<T>`, where the type parameter `T` is specified
