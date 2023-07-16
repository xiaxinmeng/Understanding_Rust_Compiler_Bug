rust
macro_rules! repro {
    () => {
        {
            f()
        }
    }
}

#[must_use]
fn f() -> i32 {
    0
}

fn main() {
    repro!(); // not used, but no warning
}
