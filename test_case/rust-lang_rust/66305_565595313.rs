rust
#[repr(C)]
struct CppArray<T, const N: usize> {
    data: [T; N],
}
