rust
// error: T: Sized is not satisfied
fn to_byte_array<T>() -> [u8; T::SIZE] {
    panic!()
}
