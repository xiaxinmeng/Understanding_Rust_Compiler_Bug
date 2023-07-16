
error[E0507]: cannot move out of `self.option` which is behind a shared reference
 --> src/main.rs:7:9
  |
7 |         self.option.map(|x| x)
  |         ^^^^^^^^^^^
  |         |
  |         move occurs because `self.option` has type `std::option::Option<std::vec::Vec<u8>>`, which does not implement the `Copy` trait
  |         help: consider borrowing the `Option`'s content: `self.option.as_ref()`
