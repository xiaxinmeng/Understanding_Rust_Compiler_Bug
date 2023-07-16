 rust
mod foo {
    pub mod bar {}
    pub(some::path) fn bar() { println!("1"); }
}
mod baz {
    pub fn bar() { println!("2"); }
}
fn main() {
    use foo::bar; // Whether this imports the value depends on accessibility.
    use baz::*;
    bar(); // If `pub(some::path)` items are accessible here, this is "1"; otherwise, it is "2".
}
