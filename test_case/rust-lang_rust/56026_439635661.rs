plain
[01:39:08] test [debuginfo-both] debuginfo/vec.rs ... ignored
[01:39:08] 
[01:39:08] failures:
[01:39:08] 
[01:39:08] ---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
[01:39:08] NOTE: compiletest thinks it is using GDB without native rust support
[01:39:08] 
[01:39:08] error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:39:08] status: exit code: 0
[01:39:08] command: "arm-linux-androideabi-gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/gdb-pretty-struct-and-enums.debugger.script"
[01:39:08] ------------------------------------------
[01:39:08] ------------------------------------------
[01:39:08] 0xb6efda30 in ?? ()
[01:39:08] Breakpoint 1 at 0xb6f0f7ac: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 69.
[01:39:08] 
[01:39:08] Breakpoint 1, gdb_pretty_struct_and_enums::main::h44925421ce933524 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:69
[01:39:08] 69     zzz(); // #break
[01:39:08] $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:39:08] $2 = {<No data fields>}
[01:39:08] $3 = CStyleEnumVar1
[01:39:08] $4 = CStyleEnumVar2
[01:39:08] $5 = CStyleEnumVar3
[01:39:08] A debugging session is active.
[01:39:08] 
[01:39:08]  Inferior 1 [process 10377] will be killed.
[01:39:08] 
[01:39:08] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:39:08] ------------------------------------------
[01:39:08] stderr:
[01:39:08] ------------------------------------------
[01:39:08] ------------------------------------------
[01:39:08] warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
[01:39:08] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/a.
[01:39:08] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:39:08] warning: Unable to find dynamic linker breakpoint function.
[01:39:08] GDB will be unable to debug shared library initializers
[01:39:08] and track explicitly loaded dynamic code.
[01:39:08] warning: Unable to find dynamic linker breakpoint function.
[01:39:08] GDB will be unable to debug shared library initializers
[01:39:08] and track explicitly loaded dynamic code.
[01:39:08] warning: Could not load shared library symbols for 4 libraries, e.g. /system/bin/linker.
[01:39:08] Use the "info sharedlibrary" command to see the complete listing.
[01:39:08] Do you need "set solib-search-path" or "set sysroot"?
[01:39:08] ------------------------------------------
[01:39:08] 
[01:39:08] thread '[debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:39:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:39:08] test result: FAILED. 70 passed; 1 failed; 47 ignored; 0 measured; 0 filtered out
[01:39:08] 
[01:39:08] 
[01:39:08] 
[01:39:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "debuginfo-both" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:39:08] 
[01:39:08] 
[01:39:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:39:08] Build completed unsuccessfully in 1:25:47
---
travis_time:end:02491e8a:start=1542477528069904073,finish=1542477528093262690,duration=23358617
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0232ff76
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b6b1500
travis_time:start:0b6b1500
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ca456b2
$ dmesg | grep -i kill
