rust
fn main() {
    let result: Result<usize, ()> = Err(());
    result.unwrap();
}
