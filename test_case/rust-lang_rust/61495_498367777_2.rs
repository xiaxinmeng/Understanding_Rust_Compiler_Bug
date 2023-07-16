rust
mod foo {
pub unsafe extern "rust-intrinsic" fn add_with_overflow<T>(
    x: T, 
    y: T
) -> (T, bool)
}
