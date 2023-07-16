
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> sparc64-unknown-linux-gnu)
[00:47:10]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:47:10]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:47:10]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:47:10]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:47:10]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:47:28]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:47:32]    Compiling rand v0.0.0 (file:///checkout/src/librand)
[00:47:32]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:47:33]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:47:34] error[E0573]: expected type, found function `statfs64`
[00:47:34]    --> /checkout/src/rustc/libc_shim/../../liblibc/src/unix/notbsd/mod.rs:880:54
[00:47:34]     |
[00:47:34] 880 |     pub fn statfs64(path: *const ::c_char, buf: *mut statfs64) -> ::c_int;
[00:47:34]     |                                                      ^^^^^^^^ did you mean `stat64`?
[00:47:34] 
[00:47:34] error[E0573]: expected type, found function `statfs64`
[00:47:34]    --> /checkout/src/rustc/libc_shim/../../liblibc/src/unix/notbsd/mod.rs:882:45
[00:47:34]     |
[00:47:34] 882 |     pub fn fstatfs64(fd: ::c_int, buf: *mut statfs64) -> ::c_int;
[00:47:34]     |                                             ^^^^^^^^ did you mean `stat64`?
[00:47:34] 
[00:47:34] error[E0573]: expected type, found function `statvfs64`
[00:47:34]    --> /checkout/src/rustc/libc_shim/../../liblibc/src/unix/notbsd/mod.rs:883:55
[00:47:34]     |
[00:47:34] 883 |     pub fn statvfs64(path: *const ::c_char, buf: *mut statvfs64) -> ::c_int;
[00:47:34]     |                                                       ^^^^^^^^^ did you mean `statvfs`?
[00:47:34] 
[00:47:34] error[E0573]: expected type, found function `statvfs64`
[00:47:34]    --> /checkout/src/rustc/libc_shim/../../liblibc/src/unix/notbsd/mod.rs:884:46
[00:47:34]     |
[00:47:34] 884 |     pub fn fstatvfs64(fd: ::c_int, buf: *mut statvfs64) -> ::c_int;
[00:47:34]     |                                              ^^^^^^^^^ did you mean `statvfs`?
[00:47:34] 
[00:47:35] error: aborting due to 4 previous errors
[00:47:35] 
[00:47:35] error: Could not compile `libc`.
