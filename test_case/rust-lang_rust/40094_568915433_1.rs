text
error[E0507]: cannot move out of `self.0` which is behind a shared reference
  --> src/lib.rs:17:15
   |
17 |         match *self {
   |               ^^^^^ help: consider borrowing here: `&*self`
18 |             Error::Io(ref e) => Some(e),
19 |             Error::Xml(e) => Some(&e), // <- problematic arm
   |                        -
   |                        |
   |                        data moved here
   |                        move occurs because `e` has type `std::io::Error`, which does not implement the `Copy` trait

error[E0515]: cannot return value referencing local variable `e`
  --> src/lib.rs:19:30
   |
19 |             Error::Xml(e) => Some(&e), // <- problematic arm
   |                              ^^^^^--^
   |                              |    |
   |                              |    `e` is borrowed here
   |                              returns a value referencing data owned by the current function
