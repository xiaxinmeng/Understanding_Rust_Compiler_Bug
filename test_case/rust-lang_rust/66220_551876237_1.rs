rust
#[repr(transparent)]
pub struct OpaquePtr<T>(*mut std::ffi::c_void, std::marker::PhantomData<T>);
pub extern "C" fn bar(_: OpaquePtr<String>) {}
