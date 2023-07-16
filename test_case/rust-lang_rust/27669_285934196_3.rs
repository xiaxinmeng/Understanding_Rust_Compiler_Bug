rust
//use std::fmt::Write; // intentional omission

fn main() {
    let mut s = String::new();
    writeln!(s, "foo").unwrap();
}
/*
error: no method named `write_fmt` found for type `std::string::String` in the current scope
 --> main.rs:5:5
  |
5 |     writeln!(s, "foo").unwrap();
  |     ^^^^^^^^^^^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use std::fmt::Write;`
  = note: this error originates in a macro outside of the current crate
*/
