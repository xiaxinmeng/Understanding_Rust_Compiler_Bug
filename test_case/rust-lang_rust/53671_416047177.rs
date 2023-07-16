plain
[00:48:15] ....................................................................................................
[00:48:18] ....................................................................................................
[00:48:20] ......i.............................................................................................
[00:48:24] ....................................................................................................
[00:48:26] .......................................................iiiiiiiii....................................
[00:48:32] ....................................................................................................
[00:48:35] ....................................................................................................
[00:48:38] ....................................i...............................................................
[00:48:41] .....................................................................................i.i..ii........
---
[00:50:31] ....................................................................................................
[00:50:40] ....................................................................................................
[00:50:52] ....................................................................................................
[00:51:01] ....................................................................................................
[00:51:11] .............................................................F......................................
[00:51:25] ....................................................................................................
[00:51:33] ....................................................................................................
[00:51:44] ...........................................................................i........................
[00:51:54] ....................................................................................................
---
[00:54:54] ---- [run-pass] run-pass/issue-15523-big.rs stdout ----
[00:54:54] 
[00:54:54] error: compilation failed!
[00:54:54] status: exit code: 1
[00:54:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-15523-big.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-15523-big/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-15523-big/auxiliary"
[00:54:54] ------------------------------------------
[00:54:54] 
[00:54:54] ------------------------------------------
[00:54:54] stderr:
[00:54:54] stderr:
[00:54:54] ------------------------------------------
[00:54:54] error: this expression will panic at runtime
[00:54:54]   --> /checkout/src/test/run-pass/issue-15523-big.rs:31:14
[00:54:54]    |
[00:54:54] 31 |     PosMax = !(1 << 63),
[00:54:54]    |              ^^^^^^^^^^ attempt to negate with overflow
[00:54:54]    = note: #[deny(const_err)] on by default
[00:54:54] 
[00:54:54] error: aborting due to previous error
[00:54:54] 
---
[00:54:54] 
[00:54:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:54:54] 
[00:54:54] 
[00:54:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:54] 
[00:54:54] 
[00:54:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:54] Build completed unsuccessfully in 0:10:40
[00:54:54] Build completed unsuccessfully in 0:10:40
[00:54:54] Makefile:58: recipe for target 'check' failed
[00:54:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19ff6878
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:22c3ac30:start=1535297572596274392,finish=1535297572605087884,duration=8813492
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07daf4c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05d2e2a8
travis_time:start:05d2e2a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06f478d4
$ dmesg | grep -i kill
