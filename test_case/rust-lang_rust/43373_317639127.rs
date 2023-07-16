
[01:04:15] ---- ffi/c_str.rs - ffi::c_str::CStr::into_c_string (line 1003) stdout ----
[01:04:15] 	error: this feature has been stable since 1.20.0. Attribute no longer needed
[01:04:15]  --> <anon>:1:12
[01:04:15]   |
[01:04:15] 1 | #![feature(into_boxed_c_str)]
[01:04:15]   |            ^^^^^^^^^^^^^^^^
[01:04:15]   |
...
[01:04:15] failures:
[01:04:15]     ffi/c_str.rs - ffi::c_str::CStr::into_c_string (line 1003)
[01:04:15]     ffi/c_str.rs - ffi::c_str::CString::as_c_str (line 455)
[01:04:15]     ffi/c_str.rs - ffi::c_str::CString::into_boxed_c_str (line 476)
[01:04:15]     ffi/os_str.rs - ffi::os_str::OsString::into_boxed_os_str (line 254)
