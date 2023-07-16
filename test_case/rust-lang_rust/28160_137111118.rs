 rust
fn main() {
    let mut a;
    a += { a = 2; 3 };
    println!("{:?}", a);
}
