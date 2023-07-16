 rust
fn main() {
    let x: &[isize] = &[1, 2, 3];
    let _x: &[[isize]] = &[*x, *x];
}
