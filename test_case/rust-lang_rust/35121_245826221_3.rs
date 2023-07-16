 rust
enum Void {
}

let result_void: Result<T, Void> = ...;
match result_void {
    Ok(t) => ...,
    // ERROR!
}
