plain
travis_time:end:267e2032:start=1557843921610191916,finish=1557843922367655508,duration=757463592
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:00] 
[01:23:00] running 143 tests
[01:23:03] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:23:05] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:23:05] 
[01:23:05]  finished in 4.807
[01:23:05] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:07] 
[01:23:07] running 9 tests
[01:23:07] iiiiiiiii
[01:23:07] 
[01:23:07]  finished in 0.152
[01:23:07] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:23] 
[01:23:23] running 122 tests
[01:23:49] .iiiii...i.....i..i...i..i.iFi..i.iiF.i.i.....i..i....i..........iiii..........i..FiiF..i.......ii.i 100/122
[01:23:54] .i.i......iii.i.....ii
[01:23:54] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:54] failures:
[01:23:54] 
[01:23:54] 
[01:23:54] ---- [debuginfo-both] debuginfo/empty-string.rs stdout ----
[01:23:54] NOTE: compiletest thinks it is using GDB without native rust support
[01:23:54] NOTE: compiletest thinks it is using GDB version 7011001
[01:23:54] 
[01:23:54] error: line not found in debugger output: $1 = ""
[01:23:54] status: exit code: 0
[01:23:54] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string/empty-string.debugger.script"
[01:23:54] ------------------------------------------
[01:23:54] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:23:54] This is free software: you are free to change and redistribute it.
[01:23:54] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:23:54] and "show warranty" for details.
[01:23:54] This GDB was configured as "x86_64-linux-gnu".
[01:23:54] Type "show configuration" for configuration details.
[01:23:54] For bug reporting instructions, please see:
[01:23:54] <http://www.gnu.org/software/gdb/bugs/>.
[01:23:54] Find the GDB manual and other documentation resources online at:
[01:23:54] <http://www.gnu.org/software/gdb/documentation/>.
[01:23:54] For help, type "help".
[01:23:54] Type "apropos word" to search for commands related to "word".
[01:23:54] Breakpoint 1 at 0xf0f: file /checkout/src/test/debuginfo/empty-string.rs, line 32.
[01:23:54] [Thread debugging using libthread_db enabled]
[01:23:54] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:23:54] 
[01:23:54] Breakpoint 1, empty_string::main::h4739d54fd70936a5 () at /checkout/src/test/debuginfo/empty-string.rs:32
[01:23:54] 32     zzz(); // #break
[01:23:54] $1 = {vec = {buf = {ptr = {pointer = 0x1 <error: Cannot access memory at address 0x1>, _marker = {<No data fields>}}, cap = 0, a = {<No data fields>}}, len = 0}}
[01:23:54] $2 = ""
[01:23:54] A debugging session is active.
[01:23:54] 
[01:23:54]  Inferior 1 [process 32094] will be killed.
[01:23:54] 
[01:23:54] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:23:54] ------------------------------------------
[01:23:54] stderr:
[01:23:54] ------------------------------------------
[01:23:54] 
[01:23:54] 
[01:23:54] ------------------------------------------
[01:23:54] 
[01:23:54] 
[01:23:54] ---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
[01:23:54] NOTE: compiletest thinks it is using GDB without native rust support
[01:23:54] NOTE: compiletest thinks it is using GDB version 7011001
[01:23:54] 
[01:23:54] error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:23:54] status: exit code: 0
[01:23:54] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/gdb-pretty-struct-and-enums.debugger.script"
[01:23:54] ------------------------------------------
[01:23:54] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:23:54] This is free software: you are free to change and redistribute it.
[01:23:54] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:23:54] and "show warranty" for details.
[01:23:54] This GDB was configured as "x86_64-linux-gnu".
[01:23:54] Type "show configuration" for configuration details.
[01:23:54] For bug reporting instructions, please see:
[01:23:54] <http://www.gnu.org/software/gdb/bugs/>.
[01:23:54] Find the GDB manual and other documentation resources online at:
[01:23:54] <http://www.gnu.org/software/gdb/documentation/>.
[01:23:54] For help, type "help".
[01:23:54] Type "apropos word" to search for commands related to "word".
[01:23:54] Breakpoint 1 at 0xadf: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 60.
[01:23:54] [Thread debugging using libthread_db enabled]
[01:23:54] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:23:54] 
[01:23:54] Breakpoint 1, gdb_pretty_struct_and_enums::main::hb77f1d849dda9a93 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:60
[01:23:54] 60     zzz(); // #break
[01:23:54] $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:23:54] $2 = {<No data fields>}
[01:23:54] $3 = CStyleEnumVar1
[01:23:54] $4 = CStyleEnumVar2
[01:23:54] $5 = CStyleEnumVar3
[01:23:54] A debugging session is active.
[01:23:54] 
[01:23:54]  Inferior 1 [process 32167] will be killed.
[01:23:54] 
[01:23:54] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:23:54] ------------------------------------------
[01:23:54] stderr:
[01:23:54] ------------------------------------------
[01:23:54] 
[01:23:54] 
[01:23:54] ------------------------------------------
[01:23:54] 
[01:23:54] 
[01:23:54] ---- [debuginfo-both] debuginfo/pretty-huge-vec.rs stdout ----
[01:23:54] NOTE: compiletest thinks it is using GDB without native rust support
[01:23:54] NOTE: compiletest thinks it is using GDB version 7011001
[01:23:54] 
[01:23:54] error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
[01:23:54] status: exit code: 0
[01:23:54] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/pretty-huge-vec.debugger.script"
[01:23:54] ------------------------------------------
[01:23:54] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:23:54] This is free software: you are free to change and redistribute it.
[01:23:54] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:23:54] and "show warranty" for details.
[01:23:54] This GDB was configured as "x86_64-linux-gnu".
[01:23:54] Type "show configuration" for configuration details.
[01:23:54] For bug reporting instructions, please see:
[01:23:54] <http://www.gnu.org/software/gdb/bugs/>.
[01:23:54] Find the GDB manual and other documentation resources online at:
[01:23:54] <http://www.gnu.org/software/gdb/documentation/>.
[01:23:54] For help, type "help".
[01:23:54] Type "apropos word" to search for commands related to "word".
[01:23:54] Breakpoint 1 at 0x17bc: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 28.
[01:23:54] [Thread debugging using libthread_db enabled]
[01:23:54] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:23:54] 
[01:23:54] Breakpoint 1, pretty_huge_vec::main::h724d3ae1bb5250fc () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:28
[01:23:54] 28     zzz(); // #break
[01:23:54] $1 = {buf = {ptr = {pointer = 0x7fffbb511010 "", _marker = {<No data fields>}}, cap = 1000000000, a = {<No data fields>}}, len = 1000000000}
[01:23:54] $2 = {data_ptr = 0x7fffbb511010 "", length = 1000000000}
[01:23:54] A debugging session is active.
[01:23:54] 
[01:23:54]  Inferior 1 [process 399] will be killed.
[01:23:54] 
[01:23:54] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:23:54] ------------------------------------------
[01:23:54] stderr:
[01:23:54] ------------------------------------------
[01:23:54] 
[01:23:54] 
[01:23:54] ------------------------------------------
[01:23:54] 
[01:23:54] 
[01:23:54] ---- [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs stdout ----
[01:23:54] NOTE: compiletest thinks it is using GDB without native rust support
[01:23:54] NOTE: compiletest thinks it is using GDB version 7011001
[01:23:54] 
[01:23:54] error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
[01:23:54] status: exit code: 0
[01:23:54] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
[01:23:54] ------------------------------------------
[01:23:54] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] Copyright (C) 2016 Free Software Foundation, Inc.
[01:23:54] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:23:54] This is free software: you are free to change and redistribute it.
[01:23:54] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:23:54] and "show warranty" for details.
[01:23:54] This GDB was configured as "x86_64-linux-gnu".
[01:23:54] Type "show configuration" for configuration details.
[01:23:54] For bug reporting instructions, please see:
[01:23:54] <http://www.gnu.org/software/gdb/bugs/>.
[01:23:54] Find the GDB manual and other documentation resources online at:
[01:23:54] <http://www.gnu.org/software/gdb/documentation/>.
[01:23:54] For help, type "help".
[01:23:54] Type "apropos word" to search for commands related to "word".
[01:23:54] Breakpoint 1 at 0x1ac0: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
[01:23:54] [Thread debugging using libthread_db enabled]
[01:23:54] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:23:54] 
[01:23:54] Breakpoint 1, pretty_uninitialized_vec::main::h8ad4cf0b9701f4b8 () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
[01:23:54] 21     zzz(); // #break
[01:23:54] $1 = {buf = {ptr = {pointer = 0x1, _marker = {<No data fields>}}, cap = 140737479958528, a = {<No data fields>}}, len = 140737488348544}
[01:23:54] A debugging session is active.
[01:23:54] 
[01:23:54]  Inferior 1 [process 428] will be killed.
[01:23:54] 
[01:23:54] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:23:54] ------------------------------------------
[01:23:54] stderr:
[01:23:54] ------------------------------------------
[01:23:54] 
---
[01:23:54] test result: FAILED. 79 passed; 4 failed; 39 ignored; 0 measured; 0 filtered out
[01:23:54] 
[01:23:54] 
[01:23:54] 
[01:23:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:54] 
[01:23:54] 
[01:23:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:54] Build completed unsuccessfully in 0:13:09
[01:23:54] Build completed unsuccessfully in 0:13:09
[01:23:54] Makefile:48: recipe for target 'check' failed
[01:23:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:30b762c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 15:49:27 UTC 2019
