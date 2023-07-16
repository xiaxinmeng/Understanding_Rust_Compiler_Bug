rust
pub type HANDLE = *mut ::core::ffi::c_void;
pub type FindFileHandle = *mut ::core::ffi::c_void;
pub type BCRYPT_ALG_HANDLE = *mut ::core::ffi::c_void;
pub type HMODULE = *mut ::core::ffi::c_void;

pub const INVALID_HANDLE_VALUE: HANDLE = ::core::ptr::invalid_mut(!0);

#[cfg(target_pointer_width = "32")]
pub type SOCKET = u32;
#[cfg(target_pointer_width = "64")]
pub type SOCKET = u64;
