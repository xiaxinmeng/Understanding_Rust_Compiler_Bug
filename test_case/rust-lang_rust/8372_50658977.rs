 rust
fn main() {
    let mut iter = range(1i, 10);
    for i in iter {
        println!("{}", i);
        println!("{}", iter.next());
    }
}
