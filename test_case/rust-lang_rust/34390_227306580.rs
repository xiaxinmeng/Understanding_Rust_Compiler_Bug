 rust
// as sugar for:
const A: &'static [i32] = &[1, 2, 3];
fn main() { let x = (*A)[0]; ... }
