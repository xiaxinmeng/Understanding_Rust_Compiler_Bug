 rust
enum AB {
    None,
    A { a: int },
    B { b: int }
}

fn main() {
    let b = Some(B {b: 1});
    assert!(!match b {
        Some(A { a: _ }) => true,
        _ => false
    })
}
