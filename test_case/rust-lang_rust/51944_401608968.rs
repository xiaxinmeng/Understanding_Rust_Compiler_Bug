plain
[00:03:16]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:19] warning: unused variable: `ptr`
[00:03:19]     --> libcore/mem.rs:1243:20
[00:03:19]      |
[00:03:19] 1243 |     unsafe fn drop(ptr: *mut ()) {}
[00:03:19]      |                    ^^^ help: consider using `_ptr` instead
[00:03:19]      = note: #[warn(unused_variables)] on by default
[00:03:19] 
[00:03:30]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:30]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
---
[00:21:27]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:22:00] error: unused variable: `ptr`
[00:22:00]     --> libcore/mem.rs:1243:20
[00:22:00]      |
[00:22:00] 1243 |     unsafe fn drop(ptr: *mut ()) {}
[00:22:00]      |                    ^^^ help: consider using `_ptr` instead
[00:22:00]      = note: `-D unused-variables` implied by `-D warnings`
[00:22:00] 
[00:22:00] 
[00:22:01] Makefile:28: recipe for target 'all' failed
[00:22:01] make: *** [all] Error 1
