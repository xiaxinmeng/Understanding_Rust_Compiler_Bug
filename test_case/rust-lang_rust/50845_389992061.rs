plain
[00:53:03] ..................................i.................................................................
[00:53:28] .....................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:53:31] ...............
[00:53:44] ....................................................................................................
[00:54:24] .ii..............................................................i..................................
[00:54:49] ..................i.ii.....................................................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:55:18] ..........................iiiiiii...................................................................
[00:55:37] ....................................................................................................
[00:55:50] ....................................................................................................
[00:56:06] .................................................................................................
---
[01:07:45] 
[01:07:45] running 396 tests
[01:08:04] ....................................................................................................
[01:08:23] .....................................i..............................................................
[01:08:40] ............................................................................F.......................
[01:08:58] failures:
[01:08:58] 
[01:08:58] ---- string.rs - string::String::into_chars (line 1460) stdout ----
[01:08:58] ---- string.rs - string::String::into_chars (line 1460) stdout ----
[01:08:58]  error[E0658]: use of unstable library feature 'into_chars': new API
[01:08:58]  --> string.rs:1462:14
[01:08:58]   |
[01:08:58] 5 | assert_eq!(s.into_chars().count(), 13);
[01:08:58]   |
[01:08:58]   |
[01:08:58]   = help: add #![feature(into_chars)] to the crate attributes to enable
[01:08:58] thread 'string.rs - string::String::into_chars (line 1460)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:08:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:58] 
[01:08:58] 
---
[01:08:58] 
[01:08:58] error: test failed, to rerun pass '--doc'
[01:08:58] 
[01:08:58] 
[01:08:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:08:58] 
[01:08:58] 
[01:08:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:58] Build completed unsuccessfully in 0:24:21
[01:08:58] Build completed unsuccessfully in 0:24:21
[01:08:58] make: *** [check] Error 1
[01:08:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04766e80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
