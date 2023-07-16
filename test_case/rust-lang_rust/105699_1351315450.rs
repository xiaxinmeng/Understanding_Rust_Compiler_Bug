rust
fn main() {
    let a: &[i32; 2] = &[10; 2];
    let b: Option<&[i32]> = true.then(|| a);
}
