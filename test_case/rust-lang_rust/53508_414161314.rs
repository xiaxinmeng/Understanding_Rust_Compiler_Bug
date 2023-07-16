plain
[00:52:51] ....................................................................................................
[00:52:54] ....................................................................................................
[00:52:57] .....i..............................................................................................
[00:53:00] ....................................................................................................
[00:53:03] ......................................................iiiiiiiii.....................................
[00:53:08] ....................................................................................................
[00:53:12] ....................................................................................................
[00:53:15] ...................................i................................................................
[00:53:17] .....................................................................................i.i..ii........
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:01] 
[01:01:01] running 110 tests
[01:01:12] iiii.......i..i........i..i.i.............i..........iiii...........i.F..i....F.....ii.i.i.......ii.
[01:01:13] failures:
[01:01:13] 
[01:01:13] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[01:01:13] ---- [debuginfo-gdb] debuginfo/nil-enum.rs stdout ----
[01:01:13] NOTE: compiletest thinks it is using GDB without native rust support
[01:01:13] NOTE: compiletest thinks it is using GDB version 7011001
[01:01:13] 
[01:01:13] error: line not found in debugger output: $1 = {<No data fields>}
[01:01:13] status: exit code: 0
[01:01:13] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/nil-enum/nil-enum.debugger.script"
[01:01:13] ------------------------------------------
[01:01:13] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:01:13] Copyright (C) 2016 Free Software Foundation, Inc.
[01:01:13] Copyright (C) 2016 Free Software Foundation, Inc.
[01:01:13] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:01:13] This is free software: you are free to change and redistribute it.
[01:01:13] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:01:13] and "show warranty" for details.
[01:01:13] This GDB was configured as "x86_64-linux-gnu".
[01:01:13] Type "show configuration" for configuration details.
[01:01:13] For bug reporting instructions, please see:
[01:01:13] <http://www.gnu.org/software/gdb/bugs/>.
[01:01:13] Find the GDB manual and other documentation resources online at:
[01:01:13] <http://www.gnu.org/software/gdb/documentation/>.
[01:01:13] For help, type "help".
[01:01:13] Type "apropos word" to search for commands related to "word".
[01retty-std-collections.debugger.script"
[01:01:13] ------------------------------------------
[01:01:13] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:01:13] Copyright (C) 2016 Free Software Foundation, Inc.
[01:01:13] Copyright (C) 2016 Free Software Foundation, Inc.
[01:01:13] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:01:13] This is free software: you are free to change and redistribute it.
[01:01:13] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:01:13] and "show warranty" for details.
[01:01:13] This GDB was configured as "x86_64-linux-gnu".
[01:01:13] Type "show configuration" for configuration details.
[01:01:13] For bug reporting instructions, please see:
[01:01:13] <http://www.gnu.org/software/gdb/bugs/>.
[01:01:13] Find the GDB manual and other documentation resources online at:
[01:01:13] <http://www.gnu.org/software/gdb/documentation/>.
[01:01:13] For help, type "help".
[01:01:13] Type "apropos word" to search for commands related to "word".
[01:01:13] Breakpoint 1 at 0x144f6: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 57.
[01:01:13] [Thread debugging using libthread_db enabled]
[01:01:13] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:01:13] 
[01:01:13] Breakpoint 1, pretty_std_collections::main::h617474859dc5b54e () at /checkout/src/test/debuginfo/pretty-std-collections.rs:57
[01:01:13] 57     zzz(); // #break
[01:01:13] $1 = BTreeSet<i32>(len: 3)
[01:01:13] $2 = BTreeMap<i32, i32>(len: 3)
[01:01:13] $3 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
[01:01:13] A debugging session is active.
[01:01:13] 
[01:01:13]  Inferior 1 [process 15246] will be killed.
[01:01:13] 
[01:01:13] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:01:13] ------------------------------------------
[01:01:13] stderr:
[01:01:13] ------------------------------------------
[01:01:13] ------------------------------------------
[01:01:13] Python Exception <class 'gdb.error'> Cannot subscript requested type.: 
[01:01:13] Python Exception <class 'gdb.error'> Cannot subscript requested type.: 
[01:01:13] ------------------------------------------
[01:01:13] 
[01:01:13] thread '[debuginfo-gdb] debuginfo/pretty-std-collections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[01:01:13] 
---
[01:01:13] 
[01:01:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:01:13] 
[01:01:13] 
[01:01:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:13] 
[01:01:13] 
[01:01:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:13] Build completed unsuccessfully in 0:12:31
[01:01:13] Build completed unsuccessfully in 0:12:31
[01:01:13] make: *** [check] Error 1
[01:01:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ef71254
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0aaf1540
$ echo/bootstrap-1v3ifugz4t07z/s-f40eeanft4-1h7wppw-1iplbejydu42a
128740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
128648 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128644 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
125880 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
