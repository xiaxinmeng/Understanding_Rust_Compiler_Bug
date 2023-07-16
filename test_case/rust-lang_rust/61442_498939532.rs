rust
fn main() {
    let mut a = String::new();
    a += { let b = "Hello"; b };
}
