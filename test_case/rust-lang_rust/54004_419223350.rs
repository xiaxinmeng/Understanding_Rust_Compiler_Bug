plain
[00:50:46] ....................................................................................................
[00:50:50] ....................................................................................................
[00:50:52] ................................i...................................................................
[00:50:55] ....................................................................................................
[00:50:58] .................................................................................iiiiiiiii..........
[00:51:04] ...ii...............................................................................................
[00:51:08] ....................................................................................................
[00:51:11] ..............................................................i.....................................
[00:51:14] ....................................................................................................
---
[00:51:45] ........................................................................................i...........
[00:51:48] ....................................................................................................
[00:51:52] ....................................................................................................
[00:51:54] ....................................................................................................
[00:51:57] .i.ii.ii..ii............................i...........................................................
[00:51:57] test result: ok. 4137 passed; 0 failed; 64 ignored; 0 measured; 0 filtered out
[00:51:57] 
[00:51:57]  finished in 127.128
[00:51:57] travis_fold:end:test_ui_nll
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:39] 
[00:58:39] running 110 tests
[00:58:51] iiii.....F.i..i........i..i.i...F.....F..Fi..........iiii...........i....i..........ii.i.i...F...ii.
ne at:
[00:58:51] <http://www.gnu.org/software/gdb/documentation/>.
[00:58:51] For help, type "help".
[00:58:51] Type "apropos word" to search for commands related to "word".
[00:58:51] Breakpoint 1 at 0xb12: file /checkout/src/test/debuginfo/borrowed-enum.rs, line 81.
[00:58:51] [Thread debugging using libthread_db enabled]
[00:58:51] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:58:51] 
[00:58:51] Breakpoint 1, borrowed_enum::main::hfdf2cbe4fde99803 () at /checkout/src/test/debuginfo/borrowed-enum.rs:81
[00:58:51] 81     zzz(); // #break
[00:58:51] $1 = {TheA = {RUST$ENUM$DISR = TheA, x = 0, y = 8970181431921507452}, TheB = {RUST$ENUM$DISR = TheA, __0 = 8970181431921507452, __1 = 32767, __2 = 0}}
[00:58:51] $2 = {TheA = {RUST$ENUM$DISR = TheB, x = 140733479719185, y = 0}, TheB = {RUST$ENUM$DISR = TheB, __0 = 0, __1 = 286331153, __2 = 286331153}}
[00:58:51] $3 = {TheOnlyCase = {__0 = 4820353753753434}}
[00:58:51] A debugging session is active.
[00:58:51] 
[00:58:51]  Inferior 1 [process 14603] will be killed.
[00:58:51] 
[00:58:51] Quit anyway? (y or n) [answered Y; input not from terminal]
[00:58:51] ------------------------------------------
[00:58:51] stderr:
[00:58:51] ------------------------------------------
[00:58:51] 
---
[00:58:51] 
[00:58:51] ---- [debuginfo-gdb] debuginfo/generic-enum-with:58:51] 
[00:58:51] thread '[debuginfo-gdb] debuginfo/generic-enum-with-different-disr-sizes.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:58:51] 
[00:58:51] ---- [debuginfo-gdb] debuginfo/generic-struct-style-enum.rs stdout ----
[00:58:51] NOTE: compiletest thinks it is using GDB without native rust support
[00:58:51] NOTE: compiletest thinks it is using GDB version 7011001
[00:58:51] 
[00:58:51] error: line not found in debugger output: $1 = {{RUST$ENUM$DISR = Case1, a = 0, b = 31868, c = 31868, d = 31868, e = 31868}, {RUST$ENUM$DISR = Case1, [...]}, {RUST$ENUM$DISR = Case1, [...]}}
[00:58:51] status: exit code: 0
[00:58:51] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generic-struct-style-enum/generic-struct-style-enum.debugger.script"
[00:58:51] ------------------------------------------
[00:58:51] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[00:58:51] Copyright (C) 2016 Free Software Foundation, Inc.
[00:58:51] Copyright (C) 2016 Free Software Foundation, Inc.
[00:58:51] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:58:51] This is free software: you are free to change and redistribute it.
[00:58:51] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[00:58:51] and "show warranty" for details.
[00:58:51] This GDB was configured as "x86_64-linux-gnu".
[00:58:51] Type "show configuration" for configuration details.
[00:58:51] For bug reporting instructions, please see:
[00:58:51] <http://www.gnu.org/software/gdb/bugs/>.
[00:58:51] Find the GDB manual and other documentation resources online at:
[00:58:51] <http://www.gnu.org/software/gdb/documentation/>.
[00:58:51] For help, type "help".
[00:58:51] Type "apropos word" to search for commands related to "word".
[00:58:51] Breakpoint 1 at 0x9f7: file /checkout/src/test/debuginfo/generic-struct-style-enum.rs, line 86.
[00:58:51] [Thread debugging using libthread_db enabled]
[00:58:51] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:58:51] 
[00:58:51] Breakpoint 1, generic_struct_style_enum::main::h6baa2d814575bd09 () at /checkout/src/test/debuginfo/generic-struct-style-enum.rs:86
[00:58:51] 86     zzz(); // #break
[00:58:51] $1 = {Case1 = {RUST$ENUM$DISR = Case1, a = 0, b = 31868, c = 31868, d = 31868, e = 31868}, Case2 = {RUST$ENUM$DISR = Case1, a = 0, b = 2088533116, c = 4137712764}, Case3 = {RUST$ENUM$DISR = Case1, a = 140737331100796, b = 0}}
[00:58:51] $2 = {Case1 = {RUST$ENUM$DISR = Case2, a = 0, b = -2133, c = 4369, d = 4369, e = 4369}, Case2 = {RUST$ENUM$DISR = Case2, a = 0, b = 286331153, c = 286331153}, Case3 = {RUST$ENUM$DISR = Case2, a = 140733479719185, b = 0}}
[00:58:51] $3 = {Case1 = {RUST$ENUM$DISR = Case3, a = 6438275382588823897, b = 63405, c = 32767, d = 0, e = 0}, Case2 = {RUST$ENUM$DISR = Case3, a = 6438275382588823897, b = 32767, c = 0}, Case3 = {RUST$ENUM$DISR = Case3, a = 0, b = 6438275382588823897}}
[00:58:51] $4 = {TheOnlyCase = {a = -1}}
[00:58:51] A debugging session is active.
[00:58:51] 
[00:58:51]  Inferior 1 [process 15075] will be killed.
[00:58:51] 
[00:58:51] Quit anyway? (y or n) [answered Y; input not from terminal]
[00:58:51] 
[00:58:51] ------------------------------ug reporting instructions, please see:
[00:58:51] <http://www.gnu.org/software/gdb/bugs/>.
[00:58:51] Find the GDB manual and other documentation resources online at:
[00:58:51] <http://www.gnu.org/software/gdb/documentation/>.
[00:58:51] For help, type "help".
[00:58:51] Type "apropos word" to search for commands related to "word".
[00:58:51] Breakpoint 1 at 0x9f8: file /checkout/src/test/debuginfo/generic-tuple-style-enum.rs, line 104.
[00:58:51] [Thread debugging using libthread_db enabled]
[00:58:51] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:58:51] 
[00:58:51] Breakpoint 1, generic_tuple_style_enum::main::h0ff66a8041e01145 () at /checkout/src/test/debuginfo/generic-tuple-style-enum.rs:104
[00:58:51] 104     zzz(); // #break
[00:58:51] $1 = {Case1 = {RUST$ENUM$DISR = Case1, __0 = 0, __1 = 31868, __2 = 31868, __3 = 31868, __4 = 31868}, Case2 = {RUST$ENUM$DISR = Case1, __0 = 0, __1 = 2088533116, __2 = 4158487676}, Case3 = {RUST$ENUM$DISR = Case1, __0 = 140737351875708, __1 = 0}}
[00:58:51] $2 = {Case1 = {RUST$ENUM$DISR = Case2, __0 = 0, __1 = -2400, __2 = 4369, __3 = 4369, __4 = 4369}, Case2 = {RUST$ENUM$DISR = Case2, __0 = 0, __1 = 286331153, __2 = 286331153}, Case3 = {RUST$ENUM$DISR = Case2, __0 = 140733479719185, __1 = 0}}
[00:58:51] $3 = {Case1 = {RUST$ENUM$DISR = Case3, __0 = 6438275382588823897, __1 = -2131, __2 = 32767, __3 = 0, __4 = 0}, Case2 = {RUST$ENUM$DISR = Case3, __0 = 6438275382588823897, __1 = 32767, __2 = 0}, Case3 = {RUST$ENUM$DISR = Case3, __0 = 0, __1 = 6438275382588823897}}
[00:58:51] $4 = {TheOnlyCase = {__0 = -1}}
[00:58:51] A debugging session is ax-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:51] 
[00:58:51] 
[00:58:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:51] Build completed unsuccessfully in 0:12:16
[00:58:51] Build completed unsuccessfully in 0:12:16
[00:58:51] make: *** [check] Error 1
[00:58:51] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06cbc26b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e4653d4:start=1536264235832955119,finish=1536264235841468195,duration=8513076
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11ebf6c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/core
