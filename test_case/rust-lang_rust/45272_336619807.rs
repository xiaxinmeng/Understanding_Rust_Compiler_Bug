
Testing libstd stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:24]    Compiling rand v0.0.0 (file:///checkout/src/librand)
[00:56:24]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:56:24]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:56:24]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:56:25]    Compiling collections v0.0.0 (file:///checkout/src/libcollections)
[00:56:27] error[E0282]: type annotations needed
[00:56:27]     --> /checkout/src/liballoc/arc.rs:1547:21
[00:56:27]      |
[00:56:27] 1547 |             let x = Arc::from_raw(x_ptr);
[00:56:27]      |                 -   ^^^^^^^^^^^^^ cannot infer type for `A`
[00:56:27]      |                 |
[00:56:27]      |                 consider giving `x` a type
[00:56:27] 
[00:56:29]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:56:30] error: aborting due to previous error
[00:56:30] 
[00:56:30] error: Could not compile `alloc`.
[00:56:30] warning: build failed, waiting for other jobs to finish...
[00:58:07] error: build failed
