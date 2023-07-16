rust
fn data_pointer<T: ?Sized>(r: &T) -> *const u8 {
    r as *const T as *const u8
}
