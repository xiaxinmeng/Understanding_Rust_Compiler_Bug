 rust
fn main() {
    drop((match (Vec::new(), &()) { (mut v, b) => { v.push(b); v } })
         .into_iter())
}
