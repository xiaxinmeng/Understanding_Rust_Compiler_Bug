plain
travis_time:end:03e71280:start=1558020899272731503,finish=1558021150327312919,duration=251054581416
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
[01:23:44] 
[01:23:44] running 143 tests
[01:23:47] i..iii.....iii..iiii.....i......................i..i.................i......i.........ii.i..i..i.ii. 100/143
[01:23:49] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:23:49] 
[01:23:49]  finished in 4.913
[01:23:49] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:51] 
[01:23:51] running 9 tests
[01:23:51] iiiiiiiii
[01:23:51] 
[01:23:51]  finished in 0.162
[01:23:51] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:08] 
[01:24:08] running 122 tests
[01:24:34] .iiiii...i.....i..i...i..i.iFi..i.iiF.i.i.....i..i....i..........iiii..........i..FiiF..i.......ii.i 100/122
[01:24:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:24:39] .i.i......iii.i.....ii
[01:24:39] failures:
[01:24:39] 
[01:24:39] 
[01:24:39] ---- [debuginfo-both] debuginfo/empty-string.rs stdout ----
[01:24:39] NOTE: compiletest thinks it is using GDB without native rust support
[01:24:39] NOTE: compiletest thinks it is using GDB version 7011001
[01:24:39] 
[01:24:39] error: line not found in debugger output: $1 = ""
[01:24:39] status: exit code: 0
[01:24:39] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string/empty-string.debugger.script"
[01:24:39] ------------------------------------------
[01:24:39] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:24:39] This is free software: you are free to change and redistribute it.
[01:24:39] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:24:39] and "show warranty" for details.
[01:24:39] This GDB was configured as "x86_64-linux-gnu".
[01:24:39] Type "show configuration" for configuration details.
[01:24:39] For bug reporting instructions, please see:
[01:24:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:24:39] Find the GDB manual and other documentation resources online at:
[01:24:39] <http://www.gnu.org/software/gdb/documentation/>.
[01:24:39] For help, type "help".
[01:24:39] Type "apropos word" to search for commands related to "word".
[01:24:39] Breakpoint 1 at 0xf0f: file /checkout/src/test/debuginfo/empty-string.rs, line 32.
[01:24:39] [Thread debugging using libthread_db enabled]
[01:24:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:24:39] 
[01:24:39] Breakpoint 1, empty_string::main::h4739d54fd70936a5 () at /checkout/src/test/debuginfo/empty-string.rs:32
[01:24:39] 32     zzz(); // #break
[01:24:39] $1 = 
[01:24:39] $2 = ""
[01:24:39] A debugging session is active.
[01:24:39] 
[01:24:39]  Inferior 1 [process 32103] will be killed.
[01:24:39] 
[01:24:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:24:39] ------------------------------------------
[01:24:39] stderr:
[01:24:39] ------------------------------------------
[01:24:39] ------------------------------------------
[01:24:39] Python Exception <class 'NameError'> name 'valobj' is not defined: 
[01:24:39] ------------------------------------------
[01:24:39] 
[01:24:39] 
[01:24:39] ---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
[01:24:39] ---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
[01:24:39] NOTE: compiletest thinks it is using GDB without native rust support
[01:24:39] NOTE: compiletest thinks it is using GDB version 7011001
[01:24:39] 
[01:24:39] error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:24:39] status: exit code: 0
[01:24:39] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/gdb-pretty-struct-and-enums.debugger.script"
[01:24:39] ------------------------------------------
[01:24:39] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:24:39] This is free software: you are free to change and redistribute it.
[01:24:39] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:24:39] and "show warranty" for details.
[01:24:39] This GDB was configured as "x86_64-linux-gnu".
[01:24:39] Type "show configuration" for configuration details.
[01:24:39] For bug reporting instructions, please see:
[01:24:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:24:39] Find the GDB manual and other documentation resources online at:
[01:24:39] <http://www.gnu.org/software/gdb/documentation/>.
[01:24:39] For help, type "help".
[01:24:39] Type "apropos word" to search for commands related to "word".
[01:24:39] Breakpoint 1 at 0xadf: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 60.
[01:24:39] [Thread debugging using libthread_db enabled]
[01:24:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:24:39] 
[01:24:39] Breakpoint 1, gdb_pretty_struct_and_enums::main::hb77f1d849dda9a93 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:60
[01:24:39] 60     zzz(); // #break
[01:24:39] $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:24:39] $2 = {<No data fields>}
[01:24:39] $3 = CStyleEnumVar1
[01:24:39] $4 = CStyleEnumVar2
[01:24:39] $5 = CStyleEnumVar3
[01:24:39] A debugging session is active.
[01:24:39] 
[01:24:39]  Inferior 1 [process 32176] will be killed.
[01:24:39] 
[01:24:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:24:39] ------------------------------------------
[01:24:39] stderr:
[01:24:39] ------------------------------------------
[01:24:39] 
[01:24:39] 
[01:24:39] ------------------------------------------
[01:24:39] 
[01:24:39] 
[01:24:39] ---- [debuginfo-both] debuginfo/pretty-huge-vec.rs stdout ----
[01:24:39] NOTE: compiletest thinks it is using GDB without native rust support
[01:24:39] NOTE: compiletest thinks it is using GDB version 7011001
[01:24:39] 
[01:24:39] error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
[01:24:39] status: exit code: 0
[01:24:39] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/pretty-huge-vec.debugger.script"
[01:24:39] ------------------------------------------
[01:24:39] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:24:39] This is free software: you are free to change and redistribute it.
[01:24:39] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:24:39] and "show warranty" for details.
[01:24:39] This GDB was configured as "x86_64-linux-gnu".
[01:24:39] Type "show configuration" for configuration details.
[01:24:39] For bug reporting instructions, please see:
[01:24:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:24:39] Find the GDB manual and other documentation resources online at:
[01:24:39] <http://www.gnu.org/software/gdb/documentation/>.
[01:24:39] For help, type "help".
[01:24:39] Type "apropos word" to search for commands related to "word".
[01:24:39] Breakpoint 1 at 0x17bc: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 28.
[01:24:39] [Thread debugging using libthread_db enabled]
[01:24:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:24:39] 
[01:24:39] Breakpoint 1, pretty_huge_vec::main::h724d3ae1bb5250fc () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:28
[01:24:39] 28     zzz(); // #break
[01:24:39] $1 = {buf = {ptr = {pointer = 0x7fffbb511010 "", _marker = {<No data fields>}}, cap = 1000000000, a = {<No data fields>}}, len = 1000000000}
[01:24:39] $2 = {data_ptr = 0x7fffbb511010 "", length = 1000000000}
[01:24:39] A debugging session is active.
[01:24:39] 
[01:24:39]  Inferior 1 [process 408] will be killed.
[01:24:39] 
[01:24:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:24:39] ------------------------------------------
[01:24:39] stderr:
[01:24:39] ------------------------------------------
[01:24:39] 
[01:24:39] 
[01:24:39] ------------------------------------------
[01:24:39] 
[01:24:39] 
[01:24:39] ---- [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs stdout ----
[01:24:39] NOTE: compiletest thinks it is using GDB without native rust support
[01:24:39] NOTE: compiletest thinks it is using GDB version 7011001
[01:24:39] 
[01:24:39] error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
[01:24:39] status: exit code: 0
[01:24:39] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
[01:24:39] ------------------------------------------
[01:24:39] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] Copyright (C) 2016 Free Software Foundation, Inc.
[01:24:39] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:24:39] This is free software: you are free to change and redistribute it.
[01:24:39] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:24:39] and "show warranty" for details.
[01:24:39] This GDB was configured as "x86_64-linux-gnu".
[01:24:39] Type "show configuration" for configuration details.
[01:24:39] For bug reporting instructions, please see:
[01:24:39] <http://www.gnu.org/software/gdb/bugs/>.
[01:24:39] Find the GDB manual and other documentation resources online at:
[01:24:39] <http://www.gnu.org/software/gdb/documentation/>.
[01:24:39] For help, type "help".
[01:24:39] Type "apropos word" to search for commands related to "word".
[01:24:39] Breakpoint 1 at 0x1ac0: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
[01:24:39] [Thread debugging using libthread_db enabled]
[01:24:39] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:24:39] 
[01:24:39] Breakpoint 1, pretty_uninitialized_vec::main::h8ad4cf0b9701f4b8 () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
[01:24:39] 21     zzz(); // #break
[01:24:39] $1 = {buf = {ptr = {pointer = 0x1, _marker = {<No data fields>}}, cap = 140737479958528, a = {<No data fields>}}, len = 140737488348544}
[01:24:39] A debugging session is active.
[01:24:39] 
[01:24:39]  Inferior 1 [process 437] will be killed.
[01:24:39] 
[01:24:39] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:24:39] ------------------------------------------
[01:24:39] stderr:
[01:24:39] ------------------------------------------
[01:24:39] 
---
[01:24:39] test result: FAILED. 79 passed; 4 failed; 39 ignored; 0 measured; 0 filtered out
[01:24:39] 
[01:24:39] 
[01:24:39] 
[01:24:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:24:39] 
[01:24:39] 
[01:24:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:39] Build completed unsuccessfully in 0:13:34
[01:24:39] Build completed unsuccessfully in 0:13:34
[01:24:39] Makefile:48: recipe for target 'check' failed
[01:24:39] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:061d9fe1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 16 17:03:59 UTC 2019
---
travis_time:end:00703307:start=1558026241444695039,finish=1558026241450109912,duration=5414873
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0520090e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:101c4bc4
travis_time:start:101c4bc4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:040778c4
$ dmesg | grep -i kill
