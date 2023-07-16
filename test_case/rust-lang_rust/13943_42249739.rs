 rust
fn main() {
    let e: Result<int, int> = Ok(3);
    match e {
        Some(x) => x.unwrap(),
        Err(y) => {}
    }
}
