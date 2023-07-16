plain
[00:49:03] ....................................................................................................
[00:49:06] ....................................................................................................
[00:49:08] ....................................................................................................
[00:49:11] ....................................................................................................
[00:49:14] iiiiiiiii...........................................................................................
[00:49:20] ....................................................................................................
[00:49:24] .....i..............................................................................................
[00:49:26] ..............i.....................................................................................
[00:49:30] ....................................................................................................
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:49:38] 
[00:49:38] running 3056 tests
[00:49:47] ....................................................................................................
[00:49:57] ...............F...............................i....................................................
[00:50:19] ....................................................................................................
[00:50:28] ....................................................................................................
[00:50:43] ....................................................................................................
[00:50:51] ....................................................................................................
[00:50:51] ....................................................................................................
[00:51:01] ...............................................................F....................................
[00:51:24] ....................................................................................................
[00:51:34] ....................................................................................................
[00:51:41] ....................................................................................................
[00:51:50] ....................................................................................................
---
[00:55:34] ---- [run-pass] run-pass/async-await.rs stdout ----
[00:55:34] 
[00:55:34] error: compilation failed!
[00:55:34] status: exit code: 1
[00:55:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/async-await.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/async-await/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/async-await/auxiliary"
[00:55:34] ------------------------------------------
[00:55:34] 
[00:55:34] ------------------------------------------
[00:55:34] stderr:
[00:55:34] stderr:
[00:55:34] ------------------------------------------
[00:55:34] error[E0432]: unresolved import `std::task::Spawner`
[00:55:34]    |
[00:55:34]    |
[00:55:34] 25 |     Spawner, SpawnObjError,
[00:55:34]    |     ^^^^^^^ no `Spawner` in `task`. Did you mean to use `Spawn`?
[00:55:34] error: aborting due to previous error
[00:55:34] 
[00:55:34] For more information about this error, try `rustc --explain E0432`.
[00:55:34] 
---
[00:55:34] ---- [run-pass] run-pass/futures-api.rs stdout ----
[00:55:34] 
[00:55:34] error: compilation failed!
[00:55:34] status: exit code: 1
[00:55:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/futures-api.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/futures-api/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/futures-api/auxiliary"
[00:55:34] ------------------------------------------
[00:55:34] 
[00:55:34] ------------------------------------------
[00:55:34] stderr:
[00:55:34] stderr:
[00:55:34] ------------------------------------------
[00:55:34] error[E0432]: unresolved import `std::task::Spawner`
[00:55:34]    |
[00:55:34]    |
[00:55:34] 26 |     Spawner, SpawnObjError,
[00:55:34]    |     ^^^^^^^ no `Spawner` in `task`. Did you mean to use `Spawn`?
[00:55:34] error: aborting due to previous error
[00:55:34] 
[00:55:34] For more information about this error, try `rustc --explain E0432`.
[00:55:34] 
---
[00:55:34] 
[00:55:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:34] 
[00:55:34] 
[00:55:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:34] 
[00:55:34] 
[00:55:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:34] Build completed unsuccessfully in 0:09:17
[00:55:34] Build completed unsuccessfully in 0:09:17
[00:55:34] Makefile:58: recipe for target 'check' failed
[00:55:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23c982c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0182aa82:start=1533549382015331555,finish=1533549382022348192,duration=7016637
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f0c5513
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16c328fd
travis_time:start:16c328fd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:36ebdae7
$ dmesg | grep -i kill
