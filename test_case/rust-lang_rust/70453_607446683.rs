rust
#![feature(const_generics, arbitrary_enum_discriminant)]

#[repr(usize)]
enum MyWeirdOption<T> {
    None = 0,
    Some(T) = std::mem::size_of::<*mut T>(),
}
