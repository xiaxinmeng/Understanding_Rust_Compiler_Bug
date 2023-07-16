plain
[00:47:27] ....................................................................................................
[00:47:31] ....................................................................................................
[00:47:33] .i..................................................................................................
[00:47:37] ....................................................................................................
[00:47:39] ..................................................iiiiiiiii.........................................
[00:47:45] ....................................................................................................
[00:47:49] ....................................................................................................
[00:47:51] ...............................i....................................................................
[00:47:54] .................................................................................i.i..ii............
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:53] 
[00:54:53] running 111 tests
[00:55:04] iiii.......i..i........i..i..i.F...........i..........iiii...........i....i..........ii.i.i.......ii
[00:55:05] failures:
[00:55:05] 
[00:55:05] ---- [debuginfo-gdb] debuginfo/function-call.rs stdout ----
[00:55:05] ---- [debuginfo-gdb] debuginfo/function-call.rs stdout ----
[00:55:05] NOTE: compiletest thinks it is using GDB without native rust support
[00:55:05] NOTE: compiletest thinks it is using GDB version 7011001
[00:55:05] 
[00:55:05] error: line not found in debugger output: $1 = true
[00:55:05] status: exit code: 0
[00:55:05] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/function-call/function-call.debugger.script"
[00:55:05] ------------------------------------------
[00:55:05] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[00:55:05] Copyright (C) 2016 Free Software Foundation, Inc.
[00:55:05] Copyright (C) 2016 Free Software Foundation, Inc.
[00:55:05] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:55:05] This is free software: you are free to change and redistribute it.
[00:55:05] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[00:55:05] and "show warranty" for details.
[00:55:05] This GDB was configured as "x86_64-linux-gnu".
[00:55:05] Type "show configuration" for configuration details.
[00:55:05] For bug reporting instructions, please see:
[00:55:05] <http://www.gnu.org/software/gdb/bugs/>.
[00:55:05] Find the GDB manual and other documentation resources online at:
[00:55:05] <http://www.gnu.org/software/gdb/documentation/>.
[00:55:05] For help, type "help".
[00:55:05] Type "apropos word" to search for commands related to "word".
[00:55:05] Breakpoint 1 at 0xadf: file /checkout/src/test/debuginfo/function-call.rs, line 44.
[00:55:05] [Thread debugging using libthread_db enabled]
[00:55:05] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:55:05] 
[00:55:05] Breakpoint 1, function_call::main::h3a729fa59c47da56 () at /checkout/src/test/debuginfo/function-call.rs:44
[00:55:05] 44     zzz(); // #break
[00:55:05] ------------------------------------------
[00:55:05] stderr:
[00:55:05] ------------------------------------------
[00:55:05] ------------------------------------------
[00:55:05] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/function-call/function-call.debugger.script:9: Error in sourced command file:
[00:55:05] No symbol "fun" in current context.
[00:55:05] ------------------------------------------
[00:55:05] 
[00:55:05] thread '[debuginfo-gdb] debuginfo/function-call.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:55:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:55:05] 
[00:55:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:55:05] 
[00:55:05] 
[00:55:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:05] 
[00:55:05] 
[00:55:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:05] Build completed unsuccessfully in 0:11:31
[00:55:05] Build completed unsuccessfully in 0:11:31
[00:55:05] Makefile:58: recipe for target 'check' failed
[00:55:05] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05fefdb2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:155ba640:start=1534694384878665853,finish=1534694384885363119,duration=6697266
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ad6a438
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n
