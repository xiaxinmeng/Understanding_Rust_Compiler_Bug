plain
[00:48:39] .................................................................................................... 2200/4601
[00:48:43] ...................i................................................................................ 2300/4601
[00:48:46] .................................................................................................... 2400/4601
[00:48:50] .................................................................................................... 2500/4601
[00:48:53] ................................iiiiiiiii........................................................... 2600/4601
[00:48:59] .................................................................................................... 2800/4601
[00:49:03] .................................................................................................... 2900/4601
[00:49:06] ......................................................i............................................. 3000/4601
[00:49:09] .................................................................................................... 3100/4601
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:35] 
[01:01:35] running 111 tests
[01:01:38] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:01:38] ..iiii.....
[01:01:38] 
[01:01:38]  finished in 3.324
[01:01:38] travis_fold:end:test_codegen

---
travis_time:start:test_compile-fail-fulldeps
Check compiletest suite=compile-fail-fulldeps mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:34] 
[01:06:34] running 51 tests
a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/dropck_tarena_unsound_drop/auxiliary" "-A" "unused"
[01:07:29]     Error {
[01:07:29]         line_num: 50,
[01:07:29]         kind: Some(
[01:07:29]             Error
[01:07:29]             Error
[01:07:29]         ),
[01:07:29]         msg: "50:32: 50:47: no function or associated item named `new` found for type `arena::TypedArena<_>` in the current scope [E0599]"
[01:07:29] ]
[01:07:29] 
[01:07:29] not found errors (from test file): [
[01:07:29]     Error {
[01:07:29]     Error {
[01:07:29]         line_num: 51,
[01:07:29]         kind: Some(
[01:07:29]             Error
[01:07:29]         ),
[01:07:29]         msg: "`arena` does not live long enough"
[01:07:29] ]
[01:07:29] 
[01:07:29] thread '[compile-fail] compile-fail-fulldeps/dropck_tarena_unsound_drop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1351:13
[01:07:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:29] 
[01:07:29] ---- [compile-fail] compile-fail-fulldeps/dropck_tarena_cycle_checked.rs stdout ----
[01:07:29] 
[01:07:29] error: /checkout/src/test/compile-fail-fulldeps/dropck_tarena_cycle_checked.rs:125: unexpected error: '125:17: 125:32: no function or associated item named `new` found for type `arena::TypedArena<_>` in the current scope [E0599]'
[01:07:29] 
[01:07:29] error: /checkout/src/test/compile-fail-fulldeps/dropck_tarena_cycle_checked.rs:126: expected error not found: `arena` does not live long enough
[01:07:29] error: 1 unexpected errors found, 1 expected errors not found
[01:07:29] status: exit code: 1
[01:07:29] status: exit code: 1
[01:07:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail-fulldeps/dropck_tarena_cycle_checked.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/dropck_tarena_cycle_checked/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/dropck_tarena_cycle_checked/auxiliary" "-A" "unused"
[01:07:29]     Error {
[01:07:29]         line_num: 125,
[01:07:29]         kind: Some(
[01:07:29]             Error
[01:07:29]             Error
[01:07:29]         ),
[01:07:29]         msg: "125:17: 125:32: no function or associated item named `new` found for type `arena::TypedArena<_>` in the current scope [E0599]"
[01:07:29] ]
[01:07:29] 
[01:07:29] not found errors (from test file): [
[01:07:29]     Error {
[01:07:29]     Error {
[01:07:29]         line_num: 126,
[01:07:29]         kind: Some(
[01:07:29]             Error
[01:07:29]         ),
[01:07:29]         msg: "`arena` does not live long enough"
[01:07:29] ]
[01:07:29] 
[01:07:29] thread '[compile-fail] compile-fail-fulldeps/dropck_tarena_cycle_checked.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1351:13
[01:07:29] 
---
[01:07:29] 
[01:07:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[01:07:29] 
[01:07:29] 
[01:07:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:29] 
[01:07:29] 
[01:07:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:29] Build completed unsuccessfully in 0:23:25
[01:07:29] Build completed unsuccessfully in 0:23:25
[01:07:29] Makefile:58: recipe for target 'check' failed
[01:07:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f028b28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:040c8540:start=1539724631992606439,finish=1539724632117389981,duration=124783542
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1049dc56
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0027d242
$ dmesg | grep -i kill
