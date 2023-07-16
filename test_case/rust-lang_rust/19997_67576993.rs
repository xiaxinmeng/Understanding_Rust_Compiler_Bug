 rust
fn main() {
    let a = 0u8;
    let mut a = Some(&a);
    if let (&Some(ref b),) = (&a,) {
        a = None;
        println!("{}, {}", a, b);
    }
}
