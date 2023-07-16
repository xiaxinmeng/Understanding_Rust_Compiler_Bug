 Rust
fn foo<T = i32>(x: Option<T>) { }

fn main() {
    foo(None)
}
