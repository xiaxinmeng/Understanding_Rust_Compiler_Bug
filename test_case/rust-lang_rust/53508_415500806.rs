plain
[00:49:22] ....................................................................................................
[00:49:25] ....................................................................................................
[00:49:28] ........i...........................................................................................
[00:49:31] ....................................................................................................
[00:49:34] .........................................................iiiiiiiii..................................
[00:49:39] ....................................................................................................
[00:49:43] ....................................................................................................
[00:49:46] ......................................i.............................................................
[00:49:49] ........................................................................................i.i..ii.....
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:19] 
[00:57:19] running 110 tests
[00:57:30] iiii.......i..i........i..i.i.............i...........iiii..........i.F..i....F.....ii.i.i.......ii.
[00:57:31] failures:
[00:57:31] 
[00:57:31] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[00:57:31] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[00:57:31] NOTE: compiletest thinks it is using GDB without native rust support
[00:57:31] NOTE: compiletest thinks it is using GDB version 7011001
[00:57:31] 
[00:57:31] error: line not found in debugger output: $1 = {<No data fields>}
[00:57:31] status: exit code: 0
[00:57:31] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/nil-enum/nil-enum.debugger.script"
[00:57:31] ------------------------------------------
[00:57:31] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[00:57:31] Copyright (C) 2016 Free Software Foundation, Inc.
[00:57:31] Copyright (C) 2016 Free Software Foundation, Inc.
[00:57:31] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:57:31] This is free software: you are free to change and redistribute it.
[00:57:31] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[00:57:31] and "show warranty" for details.
[00:57:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:57:31] This GDB was configured as "x86_64-linux-gnu".
[00:57:31] Type "show configuration" for configuration details.
[00:57:31] For bug reporting instructions, please see:
[00:57:31] <http://www.gnu.org/software/gdb/bugs/>.
[00:57:31] Find the GDB manual and other documentation resources online at:
[00:57:31] <http://www.gnu.org/software/gdb/documentation/>.
[00:57:31] For help, type "help".
[00:57:31] Type "apropos word" to search for commands related to "word".
[00:57:31] Breakpoint 1 at 0xb5e: file /checkout/src/test/debuginfo/nil-enum.rs, line 41.
[00:57:31] [Thread debugging using libthread_db enabled]
[00:57:31] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:57:31] [Inferior 1 (process 15233) exited with code 0145]
[00:57:31] ------------------------------------------
[00:57:31] stderr:
[00:57:31] ------------------------------------------
[00:57:31] ------------------------------------------
[00:57:31] thread 'main' panicked at 'Attempted to instantiate uninhabited type ANilEnum using mem::zeroed', /checkout/src/libcore/mem.rs:526:5
[00:57:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:31] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/nil-enum/nil-enum.debugger.script:9: Error in sourced command file:
[00:57:31] No symbol "first" in current context.
[00:57:31] ------------------------------------------
[00:57:31] 
[00:57:31] thread '[debuginfo-gdb] debuginfo/nil-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:57:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:31] 
[00:57:31] ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
[00:57:31] NOTE: compiletest thinks it is using GDB without native rust support
[00:57:31] NOTE: compiletest thinks it is using GDB version 7011001
[00:57:31] 
[00:57:31] error: line not found in debugger output: $1 = BTreeSet<i32>(len: 3) = {3, 5, 7}
[00:57:31] status: exit code: 0
[00:57:31] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-com32>(len: 3, cap: 8) = {5, 3, 7}
[00:57:31] A debugging session is active.
[00:57:31] 
[00:57:31]  Inferior 1 [process 15413] will be killed.
[00:57:31] 
[00:57:31] Quit anyway? (y or n) [answered Y; input not from terminal]
[00:57:31] ------------------------------------------
[00:57:31] stderr:
[00:57:31] ------------------------------------------
[00:57:31] ------------------------------------------
[00:57:31] Python Exception <class 'gdb.error'> Cannot subscript requested type.: 
[00:57:31] Python Exception <class 'gdb.error'> Cannot subscript requested type.: 
[00:57:31] ------------------------------------------
[00:57:31] 
[00:57:31] thread '[debuginfo-gdb] debuginfo/pretty-std-collections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:57:31] 
---
[00:57:31] test result: FAILED. 84 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
[00:57:31] 
[00:57:31] 
[00:57:31] 
[00:57:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:31] 
[00:57:31] 
[00:57:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:31] Build completed unsuccessfully in 0:12:10
[00:57:31] Build completed unsuccessfully in 0:12:10
[00:57:31] Makefile:58: recipe for target 'check' failed
[00:57:31] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f3abcda
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:298d5800:start=1535045034615376822,finish=1535045034696368596,duration=80991774
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b1189f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01bf1338
$ dmesg | grep -i kill
