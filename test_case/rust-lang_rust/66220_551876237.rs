rust
#[repr(transparent)]
#[allow(improper_ctypes)]
pub struct Opaque<T>(T);

pub extern "C" fn foo(_: *const Opaque<String>) {}
