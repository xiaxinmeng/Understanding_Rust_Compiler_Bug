rust
fn main() {
    let b = Err(());
    if true { b } else { Ok(b) };
}
