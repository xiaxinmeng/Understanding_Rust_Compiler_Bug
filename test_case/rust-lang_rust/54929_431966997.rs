plain
[00:47:10] .................................................................................................... 300/4933
[00:47:12] .................................................................................................... 400/4933
[00:47:15] .................................................................................................... 500/4933
[00:47:19] ....................i............................................................................... 600/4933
[00:47:23] .............F...................................................................................... 700/4933
[00:47:31] ..........................................................................iiiii..................... 900/4933
[00:47:34] .................................................................................................... 1000/4933
[00:47:37] .................................................................................................... 1100/4933
[00:47:39] .................................................................................................... 1200/4933
---
[00:49:34] failures:
[00:49:34] 
[00:49:34] ---- [ui] ui/conditional-compilation/cfg-attr-syntax-validation.rs stdout ----
[00:49:34] 
[00:49:34] error: /checkout/src/test/ui/conditional-compilation/cfg-attr-syntax-validation.rs:25: unexpected error: '25:11: 25:16: literal in `cfg` predicate value must be a string [E0565]'
[00:49:34] error: 1 unexpected errors found, 0 expected errors not found
[00:49:34] status: exit code: 1
[00:49:34] status: exit code: 1
[00:49:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-syntax-validation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-syntax-validation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-syntax-validation/auxiliary" "-A" "unused"
[00:49:34]     Error {
[00:49:34]         line_num: 25,
[00:49:34]         kind: Some(
[00:49:34]             Error
[00:49:34]             Error
[00:49:34]         ),
[00:49:34]         msg: "25:11: 25:16: literal in `cfg` predicate value must be a string [E0565]"
[00:49:34] ]
[00:49:34] 
[00:49:34] thread '[ui] ui/conditional-compilation/cfg-attr-syntax-validation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[00:49:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:34] 
[00:49:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:34] 
[00:49:34] 
[00:49:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:34] 
[00:49:34] 
[00:49:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:34] Build completed unsuccessfully in 0:03:29
[00:49:34] Build completed unsuccessfully in 0:03:29
[00:49:34] Makefile:58: recipe for target 'check' failed
[00:49:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f635190
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c376004:start=1540236867030322347,finish=1540236867034409500,duration=4087153
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11cb9080
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0792a006
travis_time:start:0792a006
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ef67a4d
$ dmesg | grep -i kill
