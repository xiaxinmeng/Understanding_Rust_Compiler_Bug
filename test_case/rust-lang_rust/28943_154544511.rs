
fn test() -> Vec<u8> {
    let result = Some(5).into_iter().collect();
    &*result;
    result
}
