Rust
enum Foo { }
fn make_foo() -> Option<Foo> { None }
fn main() {
    match make_foo() { //~ ERROR non-exhaustive patterns
        None => {}
    }
}
