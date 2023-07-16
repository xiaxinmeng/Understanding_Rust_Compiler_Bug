 rust
fn f<F, R>(_: F) where F: Fn() -> R {
}

fn main() {
    f(|| -> ! { () });
}
