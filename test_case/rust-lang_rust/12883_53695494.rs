 rust
fn main() {
    let x: &[int] = &[1, 2, 3];
    let _x: &[[int]] = &[*x, *x];
}
