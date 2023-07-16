 Rust
pub fn free_and_use<T>(t: Vec<T>) -> Vec<T> {
    let result = Vec { ..t };
    drop(t); result /* or drop(result); t */
}
