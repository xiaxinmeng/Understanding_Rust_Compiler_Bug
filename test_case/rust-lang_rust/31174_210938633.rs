
src/lib.rs:29:23: 29:78 error: non-scalar cast: `fn(*mut std::os::kernel::Struct_file, *const i8, u64, *mut i64) -> i64 {rust_dev_write}` as `unsafe extern "C" fn(*mut std::os::kernel::Struct_file, *const i8, u64, *mut i64) -> i64`
src/lib.rs:29     fops.write = Some(rust_dev_write as unsafe extern "C" fn(_, _, _, _) -> _);
