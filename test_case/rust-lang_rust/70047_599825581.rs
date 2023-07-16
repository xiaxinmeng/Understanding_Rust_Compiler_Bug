rust
   Compiling playground v0.0.1 (/playground)
error[E0507]: cannot move out of `self.0` which is behind a mutable reference
  --> src/main.rs:13:15
   |
13 |         match *self {
   |               ^^^^^ help: consider borrowing here: `&*self`
14 |             MyOption::MySome(value) => {
   |                              -----
   |                              |
   |                              data moved here
   |                              move occurs because `value` has type `T`, which does not implement the `Copy` trait
