Rust
fn main() {
    let mut x = None;
    x = Some(Box::new(x)); //~ ERROR mismatched types
    //~| cyclic type of infinite size
}
