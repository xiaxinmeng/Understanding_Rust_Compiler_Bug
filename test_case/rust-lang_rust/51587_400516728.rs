plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:32] 
[00:56:32] running 94 tests
[00:58:16] ........................................F....................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
$$($$pat:pat)|+)`
[01:00:43]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:40:23
[01:00:43]    |
[01:00:43] 40 |     let mbe_matcher = quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+);
[01:00:43] 
[01:00:43] error[E0061]: this function takes 6 parameters but 5 parameters were supplied
[01:00:43] error[E0061]: this function takes 6 parameters but 5 parameters were supplied
[01:00:43]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:41:23
[01:00:43]    |
[01:00:43] 41 |       let mbe_matcher = quoted::parse(mbe_matcher.into_iter().collect(),
[01:00:43] 42 | |                                     true,
[01:00:43] 42 | |                                     true,
[01:00:43] 43 | |                                     cx.parse_sess,
[01:00:43] 44 | |                                     &Features::new(),
[01:00:43] 45 | |                                     &[]);
[01:00:43] 
[01:00:43] error: aborting due to previous error
[01:00:43] 
[01:00:43] For more information about this error, try `rustc --explain E0061`.
---
[01:00:43] 
[01:00:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:00:43] 
[01:00:43] 
[01:00:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:43] 
[01:00:43] 
[01:00:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:43] Build completed unsuccessfully in 0:19:43
[01:00:43] Build completed unsuccessfully in 0:19:43
[01:00:43] Makefile:58: recipe for target 'check' failed
[01:00:43] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ebf578
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
