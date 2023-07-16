rust
fn main() {
    let x: &[u32] = &[1u32];
    let _y = &[x, &[1, 2]];
}
