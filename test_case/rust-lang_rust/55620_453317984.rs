
error[E0507]: cannot move out of borrowed content
 --> src/main.rs:2:25
  |
2 |     if let Some(&pat) = Some(&Box::new(1)) {
  |                  ---    ^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content
  |                  |
  |                  data moved here
  |
note: move occurs because `pat` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
 --> src/main.rs:2:18
  |
2 |     if let Some(&pat) = Some(&Box::new(1)) {
  |                  ^^^
