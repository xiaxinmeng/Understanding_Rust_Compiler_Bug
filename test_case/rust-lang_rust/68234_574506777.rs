rust
pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {…}
pub const fn slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T] {…}
