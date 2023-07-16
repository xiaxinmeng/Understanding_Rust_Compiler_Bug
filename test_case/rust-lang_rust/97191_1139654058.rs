plain
    Checking addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared type `CStr`
  --> library/std/src/sys/unix/mod.rs:75:35
   |
75 |         thread::Thread::set_name(&CStr::from_bytes_with_nul_unchecked(b"main\0"));
   |
help: consider importing one of these items
   |
3  | use core::ffi::CStr;
