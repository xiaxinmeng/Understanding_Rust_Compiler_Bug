plain
[01:23:28] test [debuginfo-both] debuginfo/vec.rs ... ignored
[01:23:28] 
[01:23:28] failures:
[01:23:28] 
[01:23:28] ---- [debuginfo-both] debuginfo/pretty-std-collections.rs stdout ----
[01:23:28] NOTE: compiletest thinks it is using GDB with native rust support
[01:23:28] NOTE: compiletest thinks it is using GDB version 8002000
[01:23:28] 
[01:23:28] error: line not found in debugger output: $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
[01:23:28] status: exit code: 0
[01:23:28] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script"
[01:23:28] ------------------------------------------
[01:23:28] ------------------------------------------
[01:23:28] GNU gdb (Ubuntu 8.2-0ubuntu1) 8.2
[01:23:28] Copyright (C) 2018 Free Software Foundation, Inc.
[01:23:28] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:23:28] This is free software: you are free to change and redistribute it.
[01:23:28] There is NO WARRANTY, to the extent permitted by law.
[01:23:28] Type "show copying" and "show warranty" for details.
[01:23:28] This GDB was configured as "x86_64-linux-gnu".
[01:23:28] Type "show configuration" for configuration details.
[01:23:28] For bug reporting instructions, please see:
[01:23:28] <http://www.gnu.org/software/gdb/bugs/>.
[01:23:28] Find the GDB manual and other documentation resources online at:
[01:23:28]     <http://www.gnu.org/software/gdb/documentation/>.
[01:23:28] 
[01:23:28] For help, type "help".
[01:23:28] Type "apropos word" to search for commands related to "word".
[01:23:28] Breakpoint 1 at 0x16598: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 63.
[01:23:28] [Thread debugging using libthread_db enabled]
[01:23:28] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:23:28] 
[01:23:28] Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:63
[01:23:28] 63     zzz(); // #break
[01:23:28] $1 = BTreeSet<i32>(len: 15)
[01:23:28] $2 = BTreeMap<i32, i32>(len: 15)
[01:23:28] $3 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
[01:23:28] $4 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
[01:23:28] A debugging session is active.
[01:23:28] 
[01:23:28]  Inferior 1 [process 19756] will be killed.
[01:23:28] 
[01:23:28] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:23:28] ------------------------------------------
[01:23:28] stderr:
[01:23:28] ------------------------------------------
[01:23:28] ------------------------------------------
[01:23:28] Python Exception <class 'gdb.error'> There is no member named ptr.: 
[01:23:28] Python Exception <class 'gdb.error'> There is no member named ptr.: 
[01:23:28] ------------------------------------------
[01:23:28] 
[01:23:28] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3288:9
[01:23:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:23:28] test result: FAILED. 82 passed; 1 failed; 35 ignored; 0 measured; 0 filtered out
[01:23:28] 
[01:23:28] 
[01:23:28] 
[01:23:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:28] 
[01:23:28] 
[01:23:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:28] Build completed unsuccessfully in 1:20:57
---
travis_time:end:08063fe9:start=1548684624441231954,finish=1548684624448276888,duration=7044934
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b94923c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f95bb6c
travis_time:start:0f95bb6c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a5ff5cc
$ dmesg | grep -i kill
