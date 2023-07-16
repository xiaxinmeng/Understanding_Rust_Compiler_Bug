plain
[00:22:29]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:22:50] error: local variables in const fn are unstable
[00:22:50]     --> libcore/ptr.rs:2879:17
[00:22:50]      |
[00:22:50] 2879 |             let ptr = mem::align_of::<T>() as *mut T;
[00:22:50] 
