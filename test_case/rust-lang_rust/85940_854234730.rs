rust
macro_rules! tests {
    () => {
        || 42
    }
}
fn main() {
    || 42; // ok
    tests!(); // ICE
}
