 rust
fn overflow(n: i64) {
    if n > 0 {
        overflow(n-1)
    }
}

fn main() {
    overflow(999999999);
}
