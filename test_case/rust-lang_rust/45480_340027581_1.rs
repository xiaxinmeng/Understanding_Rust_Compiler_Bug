rust
fn main() {
    let mut result = 0;
    result += "10".parse().unwrap();  // Fails on beta/nightly
    println!("{}", result);
}
