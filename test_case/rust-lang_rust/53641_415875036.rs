plain
[01:03:00] running 2152 tests
[01:03:07] ....................................................................................................
[01:03:14] ....................................................................................................
[01:03:23] ....................................................................................................
[01:03:31] .........................................i.............F............................................
[01:03:43] ....................................................................................................
[01:03:50] ....................................................................................................
[01:03:56] ....................................................................................................
[01:04:03] ....................................................................................................
---
[01:05:26] ................................................................................i...................
[01:05:29] ............................i.......................
[01:05:29] failures:
[01:05:29] 
[01:05:29] ---- iter/mod.rs - iter::Take<I>::into_inner (line 2367) stdout ----
[01:05:29] error[E0596]: cannot borrow immutable local variable `take` as mutable
[01:05:29]  --> iter/mod.rs:2372:21
[01:05:29]   |
[01:05:29] 7 | let take = iter.take(6);
[01:05:29]   |     ---- consider changing this to `mut take`
[01:05:29] 8 | while let Some(x) = take.next() {
[01:05:29]   |                     ^^^^ cannot borrow mutably
[01:05:29] thread 'iter/mod.rs - iter::Take<I>::into_inner (line 2367)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:05:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:29] 
[01:05:29] 
---
[01:05:29] 
[01:05:29] error: test failed, to rerun pass '--doc'
[01:05:29] 
[01:05:29] 
[01:05:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:05:29] 
[01:05:29] 
[01:05:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:29] Build completed unsuccessfully in 0:21:49
[01:05:29] Build completed unsuccessfully in 0:21:49
[01:05:29] Makefile:58: recipe for target 'check' failed
[01:05:29] make: *** [check] Error 1
2943184 ./obj
2834148 ./obj/build
2225260 ./obj/build/x86_64-unknown-linux-gnu
1177504 ./.git
---
149124 ./src/llvm-emscripten/test
148688 ./obj/build/bootstrap/debug/incremental
140432 ./obj/build/x86_64-unknown-linux-gnu/test/ui
134256 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f45t5rc1dy-18o9j1y-2uutwf4193fx8
128676 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125812 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125808 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
123044 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
