rust
#[repr(C)] // To avoid field reorder.
pub struct S(i16, i32, i64);
