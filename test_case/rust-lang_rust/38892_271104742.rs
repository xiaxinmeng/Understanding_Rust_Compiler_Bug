Rust
fn main() {
    let t: &[i32] = &[][..];
    let x: Result<(i32, &[i32]), &str> = Ok((1 as i32, t));
    if let Ok((1, _)) = x {
        true
    } else {
        false
    };
}
