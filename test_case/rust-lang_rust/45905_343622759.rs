
Testing libstd stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:15]    Compiling libc v0.2.32
[00:58:15]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:58:15]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:58:15]    Compiling dlmalloc v0.0.0 (file:///checkout/src/rustc/dlmalloc_shim)
[00:58:15] error[E0464]: multiple matching crates for `libc`
[00:58:15]  --> /checkout/src/rustc/dlmalloc_shim/../../dlmalloc/src/linux.rs:1:1
[00:58:15]   |
[00:58:15] 1 | extern crate libc;
[00:58:15]   | ^^^^^^^^^^^^^^^^^^
[00:58:15]   |
[00:58:15]   = note: candidates:
[00:58:15]   = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-658d35794c10b003.rlib
[00:58:15]   = note: crate name: libc
[00:58:15]   = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-f32a17a3111b01aa.rlib
[00:58:15]   = note: crate name: libc
[00:58:15] 
[00:58:15] error[E0463]: can't find crate for `libc`
[00:58:15]  --> /checkout/src/rustc/dlmalloc_shim/../../dlmalloc/src/linux.rs:1:1
[00:58:15]   |
[00:58:15] 1 | extern crate libc;
[00:58:15]   | ^^^^^^^^^^^^^^^^^^ can't find crate
[00:58:15] 
[00:58:15] error: aborting due to 2 previous errors
[00:58:15] 
[00:58:15] error: Could not compile `dlmalloc`.
