Rust
enum Foo { }
fn make_foo() -> Option<Foo> { None }
fn main() {
    match make_foo() {
        None => {},
        Some(_) => {}, //~ WARN unreachable pattern
    }
}
