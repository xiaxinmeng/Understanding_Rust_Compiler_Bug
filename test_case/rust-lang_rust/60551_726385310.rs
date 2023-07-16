rust
#![feature(min_const_generics)]

struct WithArray<T, const SIZE: usize> {
    data: [T; SIZE]
}
