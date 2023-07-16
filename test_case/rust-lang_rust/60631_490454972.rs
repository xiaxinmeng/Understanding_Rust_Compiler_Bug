`rust
fn main() {
    let test = std::env::var("TEST").unwrap();
    println!("{}", test);
}
