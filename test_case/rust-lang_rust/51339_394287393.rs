plain
[00:39:49]            
[00:39:49]            [1] Florian Loitsch. 2010. Printing floating-point numbers quickly and
[00:39:49]             ^
[00:39:49] 
[00:39:50] warning: [into_remainder] cannot be resolved, ignoring it...
[00:39:50]     |
[00:39:50]     |
[00:39:50] 732 | /     /// Returns an iterator over `chunk_size` elements of the slice at a time.
[00:39:50] 733 | |     /// The chunks are mutable slices, and do not overlap. If `chunk_size` does
[00:39:50] 734 | |     /// not divide the length of the slice, then the last chunk will not
[00:39:50] 735 | |     /// have length `chunk_size` and can be retrieved from the [`into_remainder`]
[00:39:50] 759 | |     ///
[00:39:50] 759 | |     ///
[00:39:50] 760 | |     /// [`exact_chunks_mut`]: #method.exact_chunks_mut
[00:39:50]     |
[00:39:50]     = note: the link appears in this line:
[00:39:50]             
[00:39:50]             
[00:39:50]              have length `chunk_size` and can be retrieved from the [`into_remainder`]
[00:39:50] 
[00:39:50] warning: [x] cannot be resolved, ignoring it...
[00:39:50]   --> libcore/../stdsimd/coresimd/ppsv/api/mod.rs:1:1
[00:39:50]    |
---
[00:40:00]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:40:00]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:40:00]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:40:00]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:40:02] warning: [into_remainder] cannot be resolved, ignoring it...
[00:40:02]     |
[00:40:02]     |
[00:40:02] 732 | /     /// Returns an iterator over `chunk_size` elements of the slice at a time.
[00:40:02] 733 | |     /// The chunks are mutable slices, and do not overlap. If `chunk_size` does
[00:40:02] 734 | |     /// not divide the length of the slice, then the last chunk will not
[00:40:02] 735 | |     /// have length `chunk_size` and can be retrieved from the [`into_remainder`]
[00:40:02] 759 | |     ///
[00:40:02] 759 | |     ///
[00:40:02] 760 | |     /// [`exact_chunks_mut`]: #method.exact_chunks_mut
[00:40:02]     |
[00:40:02]     = note: the link appears in this line:
[00:40:02]             
[00:40:02]             
[00:40:02]              have length `chunk_size` and can be retrieved from the [`into_remainder`]
[00:40:02] 
[00:40:02] 
[00:40:09] warning: [into_remainder] cannot be resolved, ignoring it...
[00:40:09]     |
[00:40:09]     |
[00:40:09] 732 | /     /// Returns an iterator over `chunk_size` elements of the slice at a time.
[00:40:09] 733 | |     /// The chunks are mutable slices, and do not overlap. If `chunk_size` does
[00:40:09] 734 | |     /// not divide the length of the slice, then the last chunk will not
[00:40:09] 735 | |     /// have length `chunk_size` and can be retrieved from the [`into_remainder`]
[00:40:09] 759 | |     ///
[00:40:09] 759 | |     ///
[00:40:09] 760 | |     /// [`exact_chunks_mut`]: #method.exact_chunks_mut
[00:40:09]     |
[00:40:09]     = note: the link appears in this line:
[00:40:09]             
[00:40:09]             
[00:40:09]              have length `chunk_size` and can be retrieved from the [`into_remainder`]
[00:40:09] 
[00:40:16]     Finished release [optimized] target(s) in 1m 11.42s
[00:40:16] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:40:17]     Checking getopts v0.2.17
---
[00:43:03] ...........................................................................i........................
[00:43:08] ....................................................................................................
[00:43:13] ....................................................................................................
[00:43:19] ....................................................................................................
[00:43:24] ........i.................iiiiiiiii...................................................
[00:43:24] 
[00:43:24] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:12] ...........................................................................i........................
[00:44:17] ....................................................................................................
[00:44:21] ....................................................................................................
[00:44:27] ....................................................................................................
[00:44:31] ........i.................iiiiiiiii...................................................
[00:44:31] 
[00:44:31]  finished in 67.394
[00:44:31] travis_fold:end:test_ui_nll

---
[01:20:16] travis_fold:end:stage0-linkchecker

[01:20:16] travis_time:end:stage0-linkchecker:start=1528103521715620406,finish=1528103524480752033,duration=2765131627

[01:20:24] std/vec/struct.Vec.html:985: broken link - std/std/slice/struct.ExactChunks.html
[01:20:24] std/vec/struct.Vec.html:1025: broken link - std/std/slice/struct.ExactChunksMut.html
[01:20:24] std/slice/index.html:193: broken link fragment `#method.into_remainder` pointing to `std/slice/index.html`
[01:20:28] core/slice/index.html:147: broken link fragment `#method.into_remainder` pointing to `core/slice/index.html`
[01:20:33] alloc/slice/index.html:193: broken link fragment `#method.into_remainder` pointing to `alloc/slice/index.html`
[01:20:33] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:20:33] 
[01:20:33] 
[01:20:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:20:33] expected success, got: exit code: 101
[01:20:33] expected success, got: exit code: 101
[01:20:33] 
[01:20:33] 
[01:20:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:33] Build completed unsuccessfully in 0:39:38
[01:20:33] Makefile:58: recipe for target 'check' failed
[01:20:33] make: *** [check] Error 1
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34816 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34372 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
