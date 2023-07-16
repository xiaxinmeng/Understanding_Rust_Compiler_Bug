 rust
fn main() {
    let mut a = 5;
    let b = (a, a = 6, a, a * (a, a = 7).0);
    println!("{:?}", b);
}
