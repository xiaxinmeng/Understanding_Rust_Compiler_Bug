plain
travis_time:end:29dee5c6:start=1550798865224776861,finish=1550798866250241594,duration=1025464733
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:16] 
[01:13:16] running 119 tests
[01:13:45] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...iF...i.......ii.i.i. 100/119
[01:13:50] i......iii.i.....ii
[01:13:50] 
[01:13:50] ---- [debuginfo-both] debuginfo/pretty-std.rs stdout ----
[01:13:50] ---- [debuginfo-both] debuginfo/pretty-std.rs stdout ----
[01:13:50] NOTE: compiletest thinks it is using GDB without native rust support
[01:13:50] NOTE: compiletest thinks it is using GDB version 7011001
[01:13:50] 
[01:13:50] error: line not found in debugger output: $5 = core::option::Option<i16>::Some(8)
[01:13:50] status: exit code: 0
[01:13:50] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std/pretty-std.debugger.script"
[01:13:50] ------------------------------------------
[01:13:50] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:13:50] Copyright (C) 2016 Free Software Foundation, Inc.
[01:13:50] Copyright (C) 2016 Free Software Foundation, Inc.
[01:13:50] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:13:50] This is free software: you are free to change and redistribute it.
[01:13:50] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:13:50] and "show warranty" for details.
[01:13:50] This GDB was configured as "x86_64-linux-gnu".
[01:13:50] Type "show configuration" for configuration details.
[01:13:50] For bug reporting instructions, please see:
[01:13:50] <http://www.gnu.org/software/gdb/bugs/>.
[01:13:50] Find the GDB manual and other documentation resources online at:
[01:13:50] <http://www.gnu.org/software/gdb/documentation/>.
[01:13:50] For help, type "help".
[01:13:50] Type "apropos word" to search for commands related to "word".
[01:13:50] Breakpoint 1 at 0x31a3: file /checkout/src/test/debuginfo/pretty-std.rs, line 100.
[01:13:50] [Thread debugging using libthread_db enabled]
[01:13:50] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:13:50] 
[01:13:50] Breakpoint 1, pretty_std::main::hd868472729f88994 () at /checkout/src/test/debuginfo/pretty-std.rs:100
[01:13:50] 100     zzz(); // #break
[01:13:50] $1 = &[i32](len: 4) = {0, 1, 2, 3}
[01:13:50] $2 = Vec<u64>(len: 4, cap: 4) = {4, 5, 6, 7}
[01:13:50] $3 = "IAMA string slice!"
[01:13:50] $4 = "IAMA string!"
[01:13:50] $5 = Some = {8}
[01:13:50] $6 = None
[01:13:50] $7 = "IAMA OS string \360\237\230\203"
[01:13:50] $8 = {RUST$ENCODED$ENUM$0$None = Some = {"IAMA optional string!"}}
[01:13:50] $9 = {RUST$ENCODED$ENUM$0$None = Some = {"IAMA "...}}
[01:13:50] $10 = ""
[01:13:50] A debugging session is active.
[01:13:50] 
[01:13:50]  Inferior 1 [process 761] will be killed.
[01:13:50] 
[01:13:50] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:13:50] ------------------------------------------
[01:13:50] stderr:
[01:13:50] ------------------------------------------
[01:13:50] ------------------------------------------
[01:13:50] Python Exception <class 'gdb.error'> Cannot convert value to long.: 
[01:13:50] Python Exception <class 'gdb.error'> Cannot convert value to long.: 
[01:13:50] Python Exception <class 'gdb.error'> Cannot convert value to long.: 
[01:13:50] Python Exception <class 'gdb.error'> Cannot convert value to long.: 
[01:13:50] ------------------------------------------
[01:13:50] 
[01:13:50] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:13:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:13:50] 
[01:13:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:50] 
[01:13:50] 
[01:13:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:50] 
[01:13:50] 
[01:13:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:50] Build completed unsuccessfully in 0:12:48
[01:13:50] Build completed unsuccessfully in 0:12:48
[01:13:50] make: *** [check] Error 1
[01:13:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13d41035
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 02:41:47 UTC 2019
---
travis_time:end:1f653120:start=1550803309094310077,finish=1550803309153947276,duration=59637199
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:36499595
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0015bd3a
$ dmesg | grep -i kill
