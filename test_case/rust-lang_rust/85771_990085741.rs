Rust
pub fn test(arg: Result<u64, u32>) {
    if arg.is_ok() {
        arg.unwrap();
    }
}
