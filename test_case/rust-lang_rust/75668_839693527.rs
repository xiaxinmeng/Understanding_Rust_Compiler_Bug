rust
struct X {
    #[warn(dead_code)]
    a: i32,
}

fn main() {
    let _ = X { a: 2 };
}
