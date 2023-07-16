 rust
use std::fmt::Write; // I had forgot this previously

fn foo() {
    let mut s = String::new();
    writeln!(s, "foo").unwrap();
}
