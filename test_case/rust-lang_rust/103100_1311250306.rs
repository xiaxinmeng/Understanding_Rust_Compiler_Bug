rust
fn foo<T: Into<String>>(input: T) -> String {
    input.into()
}

fn main() {
    foo(42);
}
