 rust
fn foo<F>(_: F) where F: for<'a> Fn(&'a str) -> &'a str
{}

fn main() {
    foo(|s| s); // ok
    foo(|s: &str| s); // not ok
}
