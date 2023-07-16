 rust
fn test() -> Vec<u8> {
    let result = Some(5).into_iter().collect();
    if false { return result; }
    &*result;
    result.clone();
    &result[..];
    result
}
