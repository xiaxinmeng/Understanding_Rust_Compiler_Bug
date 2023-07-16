plain
[00:49:19] ....................................................................................................
[00:49:29] ..................................................................................................i.
[00:49:39] ....................................................................................................
[00:49:52] ....................................................................................................
[00:50:03] ........................................................................F...........................
[00:50:25] .............................................................................i......................
[00:50:45] ....................................................................................................
[00:50:56] ....................................................................................................
[00:51:11] ...............................................ii...................................................
[00:51:11] ...............................................ii...................................................
[00:51:27] .........i....i.....................................................i...............................
[00:51:53] ....................................................................................................
[00:52:03] ....................................................................................................
[00:52:12] ....................................................................................................
[00:52:23] ....................................................................................................
ield 0 of Scalar(Scalar(Bits { size: 1, bits: 2 })) yielded ByRef(Ptr(Pointer { alloc_id: AllocId(513), offset: Size { raw: 0 } }), Align { abi_pow2: 0, pref_pow2: 3 })
[00:52:30] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
[00:52:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:30] error: aborting due to previous error
[00:52:30] 
[00:52:30] 
[00:52:30] 
[00:52:30] note: the compiler unexpectedly panicked. this is a bug.
[00:52:30] 
[00:52:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:52:30] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:52:30] 
[00:52:30] 
[00:52:30] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:52:30] 
[00:52:30] ------------------------------------------
[00:52:30] 
[00:52:30] thread '[run-pass] run-pass/match-arm-statics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
---
[00:52:30] 
[00:52:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:52:30] 
[00:52:30] 
[00:52:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:30] 
[00:52:30] 
[00:52:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:30] Build completed unsuccessfully in 0:08:43
[00:52:30] Build completed unsuccessfully in 0:08:43
[00:52:30] Makefile:58: recipe for target 'check' failed
[00:52:30] make: *** [check] Error 1
2449456 ./obj
2342708 ./obj/build
1748864 ./obj/build/x86_64-unknown-linux-gnu
786436 ./src
---
145452 ./obj/build/bootstrap/debug/incremental
133576 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133572 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130584 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs
130580 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs/s-f3g6fjg6qj-1ufkqpd-ef3q0uwki5t3
128816 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122500 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/re
