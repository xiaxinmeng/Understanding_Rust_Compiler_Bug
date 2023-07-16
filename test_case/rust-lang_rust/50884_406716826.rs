plain
[00:56:45] running 2098 tests
[00:56:52] ....................................................................................................
[00:56:59] ....................................................................................................
[00:57:07] ....................................................................................................
[00:57:15] ....F..............F...............i................................................................
[00:57:28] ....................................................................................................
[00:57:34] ....................................................................................................
[00:57:41] ....................................................................................................
[00:57:48] ....................................................................................................
---
[00:59:03] ....................................................................................................
<i32>();
[00:59:12]   |                        ^^^^^^^^^^^^
[00:59:12]   |
[00:59:12]   = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[00:59:12] 
[00:59:12] error[E0658]: use of unstable library feature 'nonempty_iter_arith': recently added unstable API
[00:59:12]  --> iter/iterator.rs:2285:29
[00:59:12]   |
[00:59:12] 7 | let nonempty_sum = (1..=10).sum_nonempty::<i32>();
[00:59:12]   |
[00:59:12]   |
[00:59:12]   = help: add #![feature(nonempty_iter_arith)] to the crate attributes to enable
[00:59:12] thread 'iter/iterator.rs - iter::iterator::Iterator::sum_nonempty (line 2281)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[00:59:12] 
[00:59:12] 
[00:59:12] failures:
---
[00:59:12] 
[00:59:12] error: test failed, to rerun pass '--doc'
[00:59:12] 
[00:59:12] 
[00:59:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[00:59:12] 
[00:59:12] 
[00:59:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:12] Build completed unsuccessfully in 0:19:38
[00:59:12] Build completed unsuccessfully in 0:19:38
[00:59:12] make: *** [check] Error 1
[00:59:12] Makefile:58: recipe for target 'check' failed
