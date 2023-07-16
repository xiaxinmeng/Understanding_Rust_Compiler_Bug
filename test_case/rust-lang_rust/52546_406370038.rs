plain
[00:51:48] ....................................................................................................
[00:51:52] ....................................................................................................
[00:51:54] ....................................................................................................
[00:51:57] .......................................i............................................................
[00:52:00] .......F............................................................................................
[00:52:06] ....................................................................................................
[00:52:08] ......................................................................................
[00:52:08] failures:
[00:52:08] 
[00:52:08] 
[00:52:08] ---- [compile-fail] compile-fail/specialization/issue-52050.rs stdout ----
[00:52:08] 
[00:52:08] error: /checkout/src/test/compile-fail/specialization/issue-52050.rs:38: unexpected error: '38:1: 38:30: conflicting implementations of trait `IntoPyDictPointer` for type `()`: [E0119]'
[00:52:08] error: /checkout/src/test/compile-fail/specialization/issue-52050.rs:32: expected error not found: conflicting implementations
[00:52:08] 
[00:52:08] error: 1 unexpected errors found, 1 expected errors not found
[00:52:08] status: exit code: 1
[00:52:08] status: exit code: 1
[00 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:52:08] 
[00:52:08] 
[00:52:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:08] 
[00:52:08] 
[00:52:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:08] Build completed unsuccessfully in 0:09:18
[00:52:08] Build completed unsuccessfully in 0:09:18
[00:52:08] Makefile:58: recipe for target 'check' failed
[00:52:08] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:232876b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2373d156:start=1532024555954942966,finish=1532024555963555883,duration=8612917
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014dac4d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a626fb6
travis_time:start:1a626fb6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.
