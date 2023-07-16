rust
fn main() {
    let mut var = 0;
    let other = "10".parse().unwrap();  // Fails on beta/nightly
    var += other;
}
