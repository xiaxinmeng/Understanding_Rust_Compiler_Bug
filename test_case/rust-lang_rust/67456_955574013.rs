rust
// core::ptr
pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T];
pub const fn slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T];

// core::slice
pub const unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T];
pub const unsafe fn from_raw_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T];
