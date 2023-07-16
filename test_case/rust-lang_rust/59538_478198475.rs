plain
[01:32:15] test [incremental] incremental/issue-49595/issue-49595.rs ... ok
[01:32:15] test [incremental] incremental/issue-54059.rs ... ok
[01:32:15] test [incremental] incremental/issue-59523-on-implemented-is-not-unused.rs ... ok
[01:32:15] test [incremental] incremental/krate-inherent.rs ... ok
[01:32:15] test [incremental] incremental/issue-59524-layout-scalar-valid-range-is-not-unused.rs ... ok
[01:32:16] test [incremental] incremental/krate-inlined.rs ... ok
[01:32:16] test [incremental] incremental/krate_reassign_34991/main.rs ... ok
[01:32:17] test [incremental] incremental/remove_crate/main.rs ... ok
[01:32:17] test [incremental] incremental/remapped_paths_cc/main.rs ... ok
---
[01:32:59] test [debuginfo-both] debuginfo/vec.rs ... ignored
[01:32:59] 
[01:32:59] failures:
[01:32:59] 
[01:32:59] ---- [debuginfo-both] debuginfo/generic-struct.rs stdout ----
[01:32:59] NOTE: compiletest thinks it is using GDB with native rust support
[01:32:59] NOTE: compiletest thinks it is using GDB version 8002000
[01:32:59] 
[01:32:59] error: line not found in debugger output: $1 = generic_struct::AGenericStruct<i32, i32> {key: 0, value: 1}
[01:32:59] status: exit code: 0
[01:32:59] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generic-struct/generic-struct.debugger.script"
[01:32:59] ------------------------------------------
[01:32:59] ------------------------------------------
[01:32:59] GNU gdb (Ubuntu 8.2-0ubuntu1) 8.2
[01:32:59] Copyright (C) 2018 Free Software Foundation, Inc.
[01:32:59] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:32:59] This is free software: you are free to change and redistribute it.
[01:32:59] There is NO WARRANTY, to the extent permitted by law.
[01:32:59] Type "show copying" and "show warranty" for details.
[01:32:59] This GDB was configured as "x86_64-linux-gnu".
[01:32:59] Type "show configuration" for configuration details.
[01:32:59] For bug reporting instructions, please see:
[01:32:59] <http://www.gnu.org/software/gdb/bugs/>.
[01:32:59] Find the GDB manual and other documentation resources online at:
[01:32:59]     <http://www.gnu.org/software/gdb/documentation/>.
[01:32:59] 
[01:32:59] For help, type "help".
[01:32:59] Type "apropos word" to search for commands related to "word".
[01:32:59] Breakpoint 1 at 0x1318: file /checkout/src/test/debuginfo/generic-struct.rs, line 63.
[01:32:59] [Thread debugging using libthread_db enabled]
[01:32:59] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:32:59] 
[01:32:59] Breakpoint 1, generic_struct::main () at /checkout/src/test/debuginfo/generic-struct.rs:63
[01:32:59] 63     zzz(); // #break
[01:32:59] $1 = generic_struct::AGenericStruct<i32, <recursive_type>> {key: 0, value: 1}
[01:32:59] $2 = generic_struct::AGenericStruct<i32, f64> {key: 2, value: 3.5}
[01:32:59] $3 = generic_struct::AGenericStruct<f64, i32> {key: 4.5, value: 5}
[01:32:59] $4 = generic_struct::AGenericStruct<f64, generic_struct::AGenericStruct<i32, <recursive_type>>> {key: 6.5, value: generic_struct::AGenericStruct<i32, f64> {key: 7, value: 8.5}}
[01:32:59] A debugging session is active.
[01:32:59] 
[01:32:59]  Inferior 1 [process 21426] will be killed.
[01:32:59] 
[01:32:59] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:32:59] ------------------------------------------
[01:32:59] stderr:
[01:32:59] ------------------------------------------
[01:32:59] 
[01:32:59] 
[01:32:59] ------------------------------------------
[01:32:59] 
[01:32:59] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:32:59] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:32:59] 
[01:32:59] ---- [debuginfo-both] debuginfo/pretty-std-collections.rs stdout ----
[01:32:59] NOTE: compiletest thinks it is using GDB with native rust support
[01:32:59] NOTE: compiletest thinks it is using GDB version 8002000
[01:32:59] 
[01:32:59] error: line not found in debugger output: $2 = BTreeMap<i32, i32>(len: 15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}
[01:32:59] status: exit code: 0
[01:32:59] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script"
[01:32:59] ------------------------------------------
[01:32:59] ------------------------------------------
[01:32:59] GNU gdb (Ubuntu 8.2-0ubuntu1) 8.2
[01:32:59] Copyright (C) 2018 Free Software Foundation, Inc.
[01:32:59] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:32:59] This is free software: you are free to change and redistribute it.
[01:32:59] There is NO WARRANTY, to the extent permitted by law.
[01:32:59] Type "show copying" and "show warranty" for details.
[01:32:59] This GDB was configured as "x86_64-linux-gnu".
[01:32:59] Type "show configuration" for configuration details.
[01:32:59] For bug reporting instructions, please see:
[01:32:59] <http://www.gnu.org/software/gdb/bugs/>.
[01:32:59] Find the GDB manual and other documentation resources online at:
[01:32:59]     <http://www.gnu.org/software/gdb/documentation/>.
[01:32:59] 
[01:32:59] For help, type "help".
[01:32:59] Type "apropos word" to search for commands related to "word".
[01:32:59] Breakpoint 1 at 0x151fa: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 63.
[01:32:59] [Thread debugging using libthread_db enabled]
[01:32:59] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:32:59] 
[01:32:59] Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:63
[01:32:59] 63     zzz(); // #break
[01:32:59] $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
[01:32:59] $2 = BTreeMap<i32, <recursive_type>>(len: 15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}
[01:32:59] $3 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
[01:32:59] $4 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
[01:32:59] A debugging session is active.
[01:32:59] 
[01:32:59]  Inferior 1 [process 22034] will be killed.
[01:32:59] 
[01:32:59] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:32:59] ------------------------------------------
[01:32:59] stderr:
[01:32:59] ------------------------------------------
[01:32:59] 
---
[01:32:59] test result: FAILED. 83 passed; 2 failed; 35 ignored; 0 measured; 0 filtered out
[01:32:59] 
[01:32:59] 
[01:32:59] 
[01:32:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:32:59] 
[01:32:59] 
[01:32:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:59] Build completed unsuccessfully in 1:29:47
---
travis_time:end:03209fc4:start=1553913136531641234,finish=1553913136537806598,duration=6165364
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01cc1acc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12817428
travis_time:start:12817428
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02c84cb8
$ dmesg | grep -i kill
