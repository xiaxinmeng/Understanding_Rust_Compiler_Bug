rust
fn main() {
    let mut data = Box::new((1, 2));
    let p = &data.0;
    data.1 += 1;
}
