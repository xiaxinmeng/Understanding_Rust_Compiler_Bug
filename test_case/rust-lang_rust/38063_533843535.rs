
error[E0507]: cannot move out of `*y` which is behind a shared reference
 --> src/main.rs:7:26
  |
7 |         println!("{:?}", y.unwrap());
  |                          ^
  |                          |
  |                          move occurs because `*y` has type `std::option::Option<main::Foo>`, which does not implement the `Copy` trait
  |                          help: consider borrowing the `Option`'s content: `y.as_ref()`
