rust
enum FixedResult<T, E> {
    Ok(T, [E; 0]),
    Err(E, [T; 0]),
}
