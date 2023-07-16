 rust
fn main() {
    let mut a = 0;
    macro_rules! b {
        (-$x:expr) => a
    }
    b![-1] = b![-0];
}
