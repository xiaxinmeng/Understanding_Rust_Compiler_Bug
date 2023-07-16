
error[E0515]: cannot return value referencing function parameter `x`
 --> src/main.rs:7:21
  |
7 |     c = (*b).map(|x|x.as_slice()); // cannot move `b` out of borrowed content
  |                     -^^^^^^^^^^^
  |                     |
  |                     returns a value referencing data owned by the current function
  |                     `x` is borrowed here

error[E0507]: cannot move out of `*b` which is behind a shared reference
 --> src/main.rs:7:9
  |
7 |     c = (*b).map(|x|x.as_slice()); // cannot move `b` out of borrowed content
  |         ^^^^
  |         |
  |         move occurs because `*b` has type `std::option::Option<std::vec::Vec<u8>>`, which does not implement the `Copy` trait
  |         help: consider borrowing the `Option`'s content: `(*b).as_ref()`
