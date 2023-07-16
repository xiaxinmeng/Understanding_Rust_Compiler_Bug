 rust
fn _ensure_buffer_is_object_safe<T: Buffer>(x: &T) -> &Buffer {
    x as &Buffer
}
