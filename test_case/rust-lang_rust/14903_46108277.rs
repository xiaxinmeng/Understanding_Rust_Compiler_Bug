 rust
pub struct FnPtr<..A, R> {
    f: extern "system" fn(..A) -> R,
    is_loaded: bool
}
