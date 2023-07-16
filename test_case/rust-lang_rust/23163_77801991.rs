 rust
fn is_null(&self) -> bool where T: Sized {
    *self == 0 as *const T
}
