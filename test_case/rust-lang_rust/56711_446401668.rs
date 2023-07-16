rust
fn repeat(n: i64, f: impl Fn()) {
    if n > 0 {
        f();
        for _ in 0..n {
            repeat(n - 1, f);
        }
    }
}

fn main() {
    repeat(3, || {});
}
