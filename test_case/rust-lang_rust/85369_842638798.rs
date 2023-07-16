rust
fn foo<T: Copy>(t: T) {}

fn main() {
    foo("".to_string());
}
