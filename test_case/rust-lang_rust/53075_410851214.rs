plain
[00:50:40] ....................................................................................................
[00:50:43] ....................................................................................................
[00:50:45] ....................................................................................................
[00:50:49] ....................................................................................................
[00:50:52] ......iiiiiiiii.....................................................................................
[00:50:58] ....................................................................................................
[00:51:01] ...........i........................................................................................
[00:51:04] ....................i...............................................................................
[00:51:07] ....................................................................................................
---
[01:05:56]    Compiling rand_core v0.2.1
[01:05:56]    Compiling libc v0.2.42
[01:05:57]    Compiling rand v0.5.4
[01:06:01]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:06:05] error: use of deprecated item 'rand::Rng::gen_iter': use Rng::sample_iter(&Standard) instead
[01:06:05]     |
[01:06:05]     |
[01:06:05] 408 |                 let orig: Vec<_> = rng.gen_iter::<i32>()
[01:06:05]     |
[01:06:05]     = note: `-D deprecated` implied by `-D warnings`
[01:06:05] 
[01:06:05] 
[01:06:05] error[E0599]: no method named `next_u32` found for type `rand::ThreadRng` in the current scope
[01:06:05]      |
[01:06:05]      |
[01:06:05] 1503 |                             x: rng.next_u32() % modulus,
[01:06:05]      |
[01:06:05]      = help: items from traits can only be used if the trait is in scope
[01:06:05]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:06:05]      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:06:05]              `use rand::RngCore;`
[01:06:09] error: aborting due to 2 previous errors
[01:06:09] 
[01:06:09] For more information about this error, try `rustc --explain E0599`.
[01:06:09] error: Could not compile `alloc`.
[01:06:09] error: Could not compile `alloc`.
[01:06:09] warning: build failed, waiting for other jobs to finish...
[01:06:12] error: build failed
[01:06:12] 
[01:06:12] 
[01:06:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:06:12] 
[01:06:12] 
[01:06:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:12] Build completed unsuccessfully in 0:18:23
[01:06:12] Build completed unsuccessfully in 0:18:23
[01:06:12] Makefile:58: recipe for target 'check' failed
[01:06:12] make: *** [check] Error 1
