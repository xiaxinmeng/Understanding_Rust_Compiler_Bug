plain
[00:48:42] ....................................................................................................
[00:48:45] ....................................................................................................
[00:48:48] ......i.............................................................................................
[00:48:51] ....................................................................................................
[00:48:54] .......................................................iiiiiiiii....................................
[00:49:00] ....................................................................................................
[00:49:03] ....................................................................................................
[00:49:06] ....................................i...............................................................
[00:49:09] ......................................................................................i.i..ii.......
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:55] 
[00:55:55] running 46 tests
[00:56:13] ............................FFFF..............
[00:56:13] 
[00:56:13] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[00:56:13] 
[00:56:13] error: compilation failed!
[00:56:13] error: compilation failed!
[00:56:13] status: exit code: 101
[00:56:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:56:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/match_false_edges.rs" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_false_edges" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_false_edges/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_false_edges/auxiliary"
[00:56:13] ------------------------------------------
[00:56:13] 
[00:56:13] ------------------------------------------
[00:56:13] stderr:
[00:56:13] stderr:
[00:56:13] ------------------------------------------
[00:56:13] warning: unreachable pattern
[00:56:13]   --> /checkout/src/test/mir-opt/match_false_edges.rs:45:9
[00:56:13]    |
[00:56:13] 44 |         _x => 2,
[00:56:13]    |         -- matches any value
[00:56:13] 45 |         Some(y) if guard2(y) => 3,
[00:56:13]    |         ^^^^^^^ unreachable pattern
[00:56:13]    = note: #[warn(unreachable_patterns)] on by default
[00:56:13] 
[00:56:13] warning: unreachable pattern
[00:56:13]   --> /checkout/src/test/mir-opt/match_false_edges.rs:46:9
[00:56:13]   --> /checkout/src/test/mir-opt/match_false_edges.rs:46:9
[00:56:13]    |
[00:56:13] 44 |         _x => 2,
[00:56:13]    |         -- matches any value
[00:56:13] 45 |         Some(y) if guard2(y) => 3,
[00:56:13] 46 |         _z => 4,
[00:56:13]    |         ^^ unreachable pattern
[00:56:13] 
[00:56:13] thread 'main' panicked at 'index 3 out of range for slice of length 2', libcore/slice/mod.rs:1937:5
[00:56:13] 
[00:56:13] error: internal compiler error: unexpected panic
[00:56:13] 
[00:56:13] note: the compiler unexpectedly panicked. this is a bug.
[00:56:13] note: the compiler unexpectedly panicked. this is a bug.
[00:56:13] 
[00:56:13] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:13] 
[00:56:13] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:56:13] 
[00:56:13] note: compiler flags: -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_false_edges -Z unstable-options -Z borrowck=mir -C prefer-dynamic -C rpath
[00:56:13] 
[00:56:13] ------------------------------------------
[00:56:13] 
[00:56:13] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:56:13] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:56:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:13] 
[00:56:13] ---- [mir-opt] mir-opt/nll/named-lifetimes-basic.rs stdout ----
[00:56:13] 
[00:56:13] error: compilation failed!
[00:56:13] status: exit code: 101
[00:56:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/nll/named-lifetimes-basic.rs" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/named-lifetimes-basic" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/named-lifetimes-basic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/named-lifetimes-basic/auxiliary"
[00:56:13] ------------------------------------------
[00:56:13] 
[00:56:13] ------------------------------------------
[00:56:13] stderr:
[00:56:13] stderr:
[00:56:13] ------------------------------------------
[00:56:13] thread 'main' panicked at 'index 3 out of range for slice of length 2', libcore/slice/mod.rs:1937:5
[00:56:13] 
[00:56:13] 
[00:56:13] error: internal compiler error: unexpir-opt/nll/reborrow-basic/auxiliary"
[00:56:13] ------------------------------------------
[00:56:13] 
[00:56:13] ------------------------------------------
[00:56:13] stderr:
[00:56:13] stderr:
[00:56:13] ------------------------------------------
[00:56:13] thread 'main' panicked at 'index 3 out of range for slice of length 2', libcore/slice/mod.rs:1937:5
[00:56:13] 
[00:56:13] error: internal compiler error: unexpected panic
[00:56:13] 
[00:56:13] note: the compiler unexpectedly panicked. this is a bug.
[00:56:13] note: the compiler unexpectedly panicked. this is a bug.
[00:56:13] 
[00:56:13] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:13] 
[00:56:13] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:56:13] 
[00:56:13] note: compiler flags: -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/reborrow-basic -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:56:13] 
[00:56:13] ------------------------------------------
[00:56:13] 
[00:56:13] thread '[mir-opt] mir-opt/nll/reborrow-basic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
---
[00:56:13] test result: FAILED. 42 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:13] 
[00:56:13] 
[00:56:13] 
[00:56:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:13] 
[00:56:13] 
[00:56:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:13] Build completed unsuccessfully in 0:11:35
[00:56:13] Build completed unsuccessfully in 0:11:35
[00:56:13] make: *** [check] Error 1
[00:56:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:033295da
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
