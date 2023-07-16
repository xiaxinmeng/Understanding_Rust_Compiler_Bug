 rust
fn foo(_: &[&str]) {}

fn bad(a: &str, b: &str) {
    foo(&[a, b]);
}

fn good(a: &str, b: &str) {
    foo(&[a.as_slice(), b.as_slice()]);
}

fn main() {}
