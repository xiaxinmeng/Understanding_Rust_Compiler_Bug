plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:49:43] 
[00:49:43] running 96 tests
[00:51:27] ..F.................................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
cked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:52:42] 
[00:52:42] 
[00:52:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:42] 
[00:52:42] 
[00:52:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:42] Build completed unsuccessfully in 0:13:00
[00:52:42] Build completed unsuccessfully in 0:13:00
[00:52:42] make: *** [check] Error 1
[00:52:42] Makefile:58: recipe for target 'check' failed
34560 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
34196 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
34108 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
34100 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
---
travis_time:end:05db6861:start=1533090332777025738,finish=1533090332786607674,duration=9581936
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fe79160
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1728ce96
travis_time:start:1728ce96
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00b29faa
$ dmesg | grep -i kill
