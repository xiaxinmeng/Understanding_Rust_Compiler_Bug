
error[E0308]: mismatched types
   --> src/codelab.rs:100:5
    |
xx  |  fn foo<T>(...) => T {
    |         - this type parameter
100 |   tail.val
    | 	^^^^^^^^ expected enum `std::option::Option`, found type parameter `T`
    |
    = note: expected type `std::option::Option<T>`
    = note:    found type `T`
