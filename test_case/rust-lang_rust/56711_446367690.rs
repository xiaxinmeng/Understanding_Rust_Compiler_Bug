
fn repeat(n: i64, f: impl Fn()) {
    if n > 0 {
        f();
        repeat(n - 1, f);
    }
}

fn main() {
    repeat(3, || {});
}
