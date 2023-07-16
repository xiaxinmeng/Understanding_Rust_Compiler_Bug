 rust
fn main() {
    let x = Some(box 1i);
    for &a in x.iter() {
    }
}
