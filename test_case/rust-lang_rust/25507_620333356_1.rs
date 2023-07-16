
error[E0507]: cannot move out of `y.field` which is behind a shared reference
 --> src/main.rs:9:26
  |
9 |         println!("{:?}", y.field.unwrap_or(X));
  |                          ^^^^^^^
  |                          |
  |                          move occurs because `y.field` has type `std::option::Option<X>`, which does not implement the `Copy` trait
  |                          help: consider borrowing the `Option`'s content: `y.field.as_ref()`
