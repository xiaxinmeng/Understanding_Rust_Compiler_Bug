plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Running apt-get update by default has been disabled.
You can opt into running apt-get update by setting this in your .travis.yml file:
  apt:
travis_fold:start:git.checkout
travis_time:start:00791972
$ git clone --depth=2 https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:57:20] ....................................................................................................
[00:57:26] ....................................................................................................
[00:57:32] ....................................................................................................
[00:57:39] ....................................................................................................
[00:57:45] ...F................i............................................................ii.iii.............
[00:57:58] ..........i..............................i..........................................................
[00:58:04] ....................................................................................................
[00:58:10] ............i.......................................................................................
[00:58:16] ....................................................................................................
---
[01:00:10] failures:
[01:00:10] 
[01:00:10] ---- [compile-fail] compile-fail/const-fn-not-safe-for-const.rs stdout ----
[01:00:10]  
[01:00:10] error: /checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs:20: expected error not found: E0015
[01:00:10] 
[01:00:10] error: 0 unexpected errors found, 1 expected errors not found
[01:00:10] status: exit code: 101
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:10] not found errors (from test file): [
[01:00:10]     Error {
[01:00:10]         line_num: 20,
[01:00:10]         kind: Some(
[01:00:10]         ),
[01:00:10]         ),
[01:00:10]         msg: "E0015"
[01:00:10] ]
[01:00:10] 
[01:00:10] thread '[compile-fail] compile-fail/const-fn-not-safe-for-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:00:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:00:10] 
[01:00:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:00:10] 
[01:00:10] 
[01:00:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:10] 
[01:00:10] 
[01:00:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:10] Build completed unsuccessfully in 0:16:34
[01:00:10] Build completed unsuccessfully in 0:16:34
[01:00:10] make: *** [check] Error 1
[01:00:10] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:088c003a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
