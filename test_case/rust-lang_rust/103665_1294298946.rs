rust
fn f(_: &[u8]) {
}

fn main() {
    let b = Box::new([0u8; 1000]);
    f(&*b);
}
