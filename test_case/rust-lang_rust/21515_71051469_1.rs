
godot2:servo jdm$ ./mach rustc /tmp/ffi.rs
/tmp/ffi.rs:11:17: 11:49 error: non-scalar cast: `unsafe extern "C" fn()` as `unsafe extern "C" fn()`
/tmp/ffi.rs:11         f: Some(ffi_fn as unsafe extern "C" fn()),
                               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
