plain
travis_time:end:14e11330:start=1542828866948556325,finish=1542828952978692451,duration=86030136126
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:22] .................................................................................................... 100/5043
[00:53:25] .................................................................................................... 200/5043
[00:53:28] .............................ii............................................ii...................ii.. 300/5043
[00:53:31] ..............................................................................................iii... 400/5043
[00:53:34] .....iiiiiiii.iii............................iii...........................................i........ 500/5043
[00:53:41] .................................................................................................... 700/5043
[00:53:47] ..................................................................................i...........i..... 800/5043
[00:53:51] .................................................................................................... 900/5043
[00:53:54] .iiiii..................ii.iiii..................................................................... 1000/5043
---
[00:54:31] .................................................................................................... 2200/5043
[00:54:35] .................................................................................................... 2300/5043
[00:54:39] .................................................................................................... 2400/5043
[00:54:43] .................................................................................................... 2500/5043
[00:54:46] ......................................................................................iiiiiiiii..... 2600/5043
[00:54:54] ....................................................ii.............................................. 2800/5043
[00:54:56] .................................................................................................... 2900/5043
[00:55:01] .................................................................................................... 3000/5043
[00:55:04] ................................................i................................................... 3100/5043
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:38] 
[01:09:38] running 117 tests
[01:09:41] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:09:41] i.i.....iiii.....
[01:09:41] 
[01:09:41]  finished in 3.630
[01:09:41] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:57] 
[01:09:57] running 118 tests
[01:10:22] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i...Fi...i.......ii.i.i.i 100/118
[01:10:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:10:26] ......iii.i.....ii
[01:10:26] 
[01:10:26] ---- [debuginfo-both] debuginfo/pretty-std-collections.rs stdout ----
[01:10:26] ---- [debuginfo-both] debuginfo/pretty-std-collections.rs stdout ----
[01:10:26] NOTE: compiletest thinks it is using GDB without native rust support
[01:10:26] NOTE: compiletest thinks it is using GDB version 7011001
[01:10:26] 
[01:10:26] error: line not found in debugger output: $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
[01:10:26] status: exit code: 0
[01:10:26] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script"
[01:10:26] ------------------------------------------
[01:10:26] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:10:26] Copyright (C) 2016 Free Software Foundation, Inc.
[01:10:26] Copyright (C) 2016 Free Software Foundation, Inc.
[01:10:26] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:10:26] This is free software: you are free to change and redistribute it.
[01:10:26] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:10:26] and "show warranty" for details.
[01:10:26] This GDB was configured as "x86_64-linux-gnu".
[01:10:26] Type "show configuration" for configuration details.
[01:10:26] For bug reporting instructions, please see:
[01:10:26] <http://www.gnu.org/software/gdb/bugs/>.
[01:10:26] Find the GDB manual and other documentation resources online at:
[01:10:26] <http://www.gnu.org/software/gdb/documentation/>.
[01:10:26] For help, type "help".
[01:10:26] Type "apropos word" to search for commands related to "word".
[01:10:26] Breakpoint 1 at 0x1380a: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 58.
[01:10:26] [Thread debugging using libthread_db enabled]
[01:10:26] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:10:26] 
[01:10:26] Breakpoint 1, pretty_std_collections::main::h8f4e9a7b836baf63 () at /checkout/src/test/debuginfo/pretty-std-collections.rs:58
[01:10:26] 58     zzz(); // #break
[01:10:26] $1 = BTreeSet<i32>(len: 15)
[01:10:26] $2 = BTreeMap<i32, i32>(len: 15)
[01:10:26] $3 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
[01:10:26] A debugging session is active.
[01:10:26] 
[01:10:26]  Inferior 1 [process 3868] will be killed.
[01:10:26] 
[01:10:26] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:10:26] ------------------------------------------
[01:10:26] stderr:
[01:10:26] ------------------------------------------
[01:10:26] ------------------------------------------
[01:10:26] Python Exception <class 'TypeError'> 'gdb.Value' object cannot be interpreted as an integer: 
[01:10:26] Python Exception <class 'TypeError'> 'gdb.Value' object cannot be interpreted as an integer: 
[01:10:26] ------------------------------------------
[01:10:26] 
[01:10:26] thread '[debuginfo-both] debuginfo/pretty-std-collections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:10:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:10:26] test result: FAILED. 81 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out
[01:10:26] 
[01:10:26] 
[01:10:26] 
[01:10:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:26] 
[01:10:26] 
[01:10:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:26] Build completed unsuccessfully in 0:21:07
[01:10:26] Build completed unsuccessfully in 0:21:07
[01:10:26] make: *** [check] Error 1
[01:10:26] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03299408
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 20:46:28 UTC 2018
