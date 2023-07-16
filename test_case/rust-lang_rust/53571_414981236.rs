plain
[00:48:03] ....................................................................................................
[00:48:06] ....................................................................................................
[00:48:09] ..........i.........................................................................................
[00:48:12] ....................................................................................................
[00:48:14] ...........................................................iiiiiiiii................................
[00:48:20] ....................................................................................................
[00:48:24] ....................................................................................................
[00:48:26] ........................................i...........................................................
[00:48:29] ..........................................................................................i.i..ii...
---
[01:01:28]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:01:29] error[E0433]: failed to resolve. Use of undeclared type or module `Box`
[01:01:29]     --> liballoc/collections/vec_deque.rs:2973:24
[01:01:29]      |
[01:01:29] 2973 |         dst.push_front(Box::new(1));
[01:01:29]      |                        ^^^ Use of undeclared type or module `Box`
[01:01:29] error[E0433]: failed to resolve. Use of undeclared type or module `Box`
[01:01:29]     --> liballoc/collections/vec_deque.rs:2974:24
[01:01:29]      |
[01:01:29]      |
[01:01:29] 2974 |         dst.push_front(Box::new(2));
[01:01:29]      |                        ^^^ Use of undeclared type or module `Box`
[01:01:29] error[E0433]: failed to resolve. Use of undeclared type or module `Box`
[01:01:29]     --> liballoc/collections/vec_deque.rs:2978:24
[01:01:29]      |
[01:01:29]      |
[01:01:29] 2978 |         src.push_front(Box::new(0));
[01:01:29]      |                        ^^^ Use of undeclared type or module `Box`
[01:01:32] error: aborting due to 3 previous errors
[01:01:32] 
[01:01:32] For more information about this error, try `rustc --explain E0433`.
[01:01:32] error: Could not compile `alloc`.
[01:01:32] error: Could not compile `alloc`.
[01:01:32] warning: build failed, waiting for other jobs to finish...
[01:02:06] error: build failed
[01:02:06] 
[01:02:06] 
[01:02:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:02:06] 
[01:02:06] 
4776388 .
2940264 ./obj
