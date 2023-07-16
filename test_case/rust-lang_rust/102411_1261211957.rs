rust
fn main() {
    let x: Result<i32, i32> = Ok(42);
    let (Ok(_) | Err(_)) = x;
}
