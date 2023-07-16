plain
[01:29:20] test [debuginfo-both] debuginfo/vec.rs ... ignored
[01:29:20] 
[01:29:20] failures:
[01:29:20] 
[01:29:20] ---- [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs stdout ----
[01:29:20] NOTE: compiletest thinks it is using GDB with native rust support
[01:29:20] NOTE: compiletest thinks it is using GDB version 8002091
[01:29:20] error: gdb failed to execute
[01:29:20] status: exit code: 1
[01:29:20] status: exit code: 1
[01:29:20] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
[01:29:20] ------------------------------------------
[01:29:20] ------------------------------------------
[01:29:20] GNU gdb (Ubuntu 8.2.91.20190405-0ubuntu3) 8.2.91.20190405-git
[01:29:20] Copyright (C) 2019 Free Software Foundation, Inc.
[01:29:20] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:29:20] This is free software: you are free to change and redistribute it.
[01:29:20] There is NO WARRANTY, to the extent permitted by law.
[01:29:20] Type "show copying" and "show warranty" for details.
[01:29:20] This GDB was configured as "x86_64-linux-gnu".
[01:29:20] Type "show configuration" for configuration details.
[01:29:20] For bug reporting instructions, please see:
[01:29:20] <http://www.gnu.org/software/gdb/bugs/>.
[01:29:20] Find the GDB manual and other documentation resources online at:
[01:29:20]     <http://www.gnu.org/software/gdb/documentation/>.
[01:29:20] 
[01:29:20] For help, type "help".
[01:29:20] Type "apropos word" to search for commands related to "word".
[01:29:20] Breakpoint 1 at 0x20a4: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
[01:29:20] [Thread debugging using libthread_db enabled]
[01:29:20] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:29:20] 
[01:29:20] Breakpoint 1, pretty_uninitialized_vec::main () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
[01:29:20] 21     zzz(); // #break
[01:29:20] $1 = Vec<i32>(len: 140737488348064, cap: 0) = {
[01:29:20] ------------------------------------------
[01:29:20] stderr:
[01:29:20] ------------------------------------------
[01:29:20] ------------------------------------------
[01:29:20] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script:10: Error in sourced command file:
[01:29:20] Cannot access memory at address 0x0
[01:29:20] ------------------------------------------
[01:29:20] 
[01:29:20] 
[01:29:20] 
[01:29:20] 
[01:29:20] failures:
[01:29:20]     [debuginfo-both] debuginfo/pretty-uninitialized-vec.rs
[01:29:20] 
[01:29:20] test result: FAILED. 86 passed; 1 failed; 35 ignored; 0 measured; 0 filtered out
[01:29:20] 
[01:29:20] 
[01:29:20] 
[01:29:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-both" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:29:20] 
[01:29:20] 
[01:29:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:20] Build completed unsuccessfully in 1:26:03
---
travis_time:end:0051f588:start=1557963792342185350,finish=1557963792348710377,duration=6525027
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:026e0056
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0243cee4
travis_time:start:0243cee4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f95c56e
$ dmesg | grep -i kill
