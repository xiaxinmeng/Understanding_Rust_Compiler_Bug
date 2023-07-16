 rust
fn main() {
    let mut a = 0;
    macro_rules! b { () => {a} }
    b![] = b![];
}
