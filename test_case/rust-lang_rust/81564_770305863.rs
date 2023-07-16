rust
fn main() {
    let a = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/variable.txt"));
}
