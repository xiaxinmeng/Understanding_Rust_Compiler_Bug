 Rust
fn f(_a: &[int]) {
}

fn main() {
    static A: [int, ..0u] = [];
    f(A); // triggers assertion
}
