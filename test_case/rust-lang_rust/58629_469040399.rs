plain
[01:42:09] 
[01:42:09] failures:
[01:42:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:42:09] 
[01:42:09] ---- [debuginfo-both] debuginfo/empty-string.rs stdout ----
[01:42:09] NOTE: compiletest thinks it is using GDB without native rust support
[01:42:09] 
[01:42:09] error: line not found in debugger output: $1 = ""
[01:42:09] status: exit code: 0
[01:42:09] command: "arm-linux-androideabi-gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string/empty-string.debugger.script"
[01:42:09] ------------------------------------------
[01:42:09] ------------------------------------------
[01:42:09] 0xb6fe1a30 in ?? ()
[01:42:09] Breakpoint 1 at 0xb6ff4560: file /checkout/src/test/debuginfo/empty-string.rs, line 31.
[01:42:09] 
[01:42:09] Breakpoint 1, empty_string::main::h2741c6f3b053ffe9 () at /checkout/src/test/debuginfo/empty-string.rs:31
[01:42:09] 31     zzz(); // #break
[01:42:09] $1 = {vec = {buf = {ptr = {pointer = 0x1 <error: Cannot access memory at address 0x1>, _marker = {<No data fields>}}, cap = 0, a = {<No data fields>}}, len = 0}}
[01:42:09] $2 = {data_ptr = 0xb6ff4d09 "", length = 0}
[01:42:09] A debugging session is active.
[01:42:09] 
[01:42:09]  Inferior 1 [process 22290] will be killed.
[01:42:09] 
[01:42:09] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:42:09] ------------------------------------------
[01:42:09] stderr:
[01:42:09] ------------------------------------------
[01:42:09] ------------------------------------------
[01:42:09] warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
[01:42:09] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string/a.
[01:42:09] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:42:09] warning: Unable to find dynamic linker breakpoint function.
[01:42:09] GDB will be unable to debug shared library initializers
[01:42:09] and track explicitly loaded dynamic code.
[01:42:09] warning: Unable to find dynamic linker breakpoint function.
[01:42:09] GDB will be unable to debug shared library initializers
[01:42:09] and track explicitly loaded dynamic code.
[01:42:09] warning: Could not load shared library symbols for 4 libraries, e.g. /system/bin/linker.
[01:42:09] Use the "info sharedlibrary" command to see the complete listing.
[01:42:09] Do you need "set solib-search-path" or "set sysroot"?
[01:42:09] ------------------------------------------
[01:42:09] 
[01:42:09] thread 'main' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:42:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:42:09] test result: FAILED. 71 passed; 1 failed; 48 ignored; 0 measured; 0 filtered out
[01:42:09] 
[01:42:09] 
[01:42:09] 
[01:42:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "debuginfo-both" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:42:09] 
[01:42:09] 
[01:42:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:42:09] Build completed unsuccessfully in 1:31:55
---
travis_time:end:011d8106:start=1551631444246113397,finish=1551631444253938006,duration=7824609
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ba7c890
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18fece40
travis_time:start:18fece40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:010bc52a
$ dmesg | grep -i kill
