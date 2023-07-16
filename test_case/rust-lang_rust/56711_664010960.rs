rust
fn repeat(n: i64, f: impl FnOnce() + Copy) {
    if n > 0 {
        f();
        for _ in 0..n {
            repeat(n - 1, f);
        }
    }
}

fn main() {
    repeat(3, || {println!("hi")}); // works
    repeat(3, &|| {println!("hi")}); // also works
}
