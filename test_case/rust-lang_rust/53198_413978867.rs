Rust
fn main() {
    let mut x = vec![];
    let p = &x;
    x.push(p.len());
}
