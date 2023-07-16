rust
pub fn create_none_array<T, const N: usize>() -> [Option<T>; N] {
    [const { None }; N]
}
