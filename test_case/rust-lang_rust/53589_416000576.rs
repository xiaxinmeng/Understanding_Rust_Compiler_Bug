Rust
struct Slice<'a, T> {
    ptr: *const T,
    __phantom: Phantom<&'a T>
}
