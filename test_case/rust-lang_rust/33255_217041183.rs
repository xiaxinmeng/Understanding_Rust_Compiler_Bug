 rust
fn add<T: Add>(a: T, b: T) -> T {
    a + b
}

fn main() {
    add(200_u8, 200_u8);
}
