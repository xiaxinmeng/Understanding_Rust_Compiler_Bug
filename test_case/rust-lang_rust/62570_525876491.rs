rust
fn main() {
    let _ = || {
        Err(5)?;
        1
    };
}
