
error[E0507]: cannot move out of `self.member` which is behind a shared reference
 --> src/main.rs:7:20
  |
7 |         for foo in self.member { // line 7
  |                    ^^^^^^^^^^^ move occurs because `self.member` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait
