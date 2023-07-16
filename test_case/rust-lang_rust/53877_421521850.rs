plain
[00:53:44] ....................................................................................................
[00:53:47] ....................................................i...............................................
[00:53:50] ....................................................................................................
[00:53:53] ....................................................................................................
[00:53:55] iiiiiiiii...........................................................................................
[00:54:01] ....................................................................................................
[00:54:04] .................................................................................i..................
[00:54:07] ....................................................................................................
[00:54:10] ...................................i.i..ii..........................................................
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:19] 
[00:59:19] running 549 tests
[00:59:32] .........................F..............i...........................................................
[00:59:57] ....................................................................................................
[01:00:07] ....................................................................................................
[01:00:22] ................i...................................................................................
[01:00:22] ................i...................................................................................
type `std::pin::Pin<std::boxed::Box<Fut>>` in the current scope
[01:00:28]     |
[01:00:28]     |
[01:00:28] 157 |     assert_eq!(Poll::Pending, fut.as_pin_mut().poll(cx));
[01:00:28] 
[01:00:28] 
[01:00:28] error[E0599]: no method named `as_pin_mut` found for type `std::pin::Pin<std::boxed::Box<Fut>>` in the current scope
[01:00:28]     |
[01:00:28]     |
[01:00:28] 159 |     assert_eq!(Poll::Ready(9), fut.as_pin_mut().poll(cx));
[01:00:28] 
[01:00:28] error: aborting due to 2 previous errors
[01:00:28] 
[01:00:28] For more information about this error, try `rustc --explain E0599`.
---
[01:00:28] test result: FAILED. 546 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:00:28] 
[01:00:28] 
[01:00:28] 
[01:00:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:28] 
[01:00:28] 
[01:00:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:28] Build completed unsuccessfully in 0:15:24
[01:00:28] Build completed unsuccessfully in 0:15:24
[01:00:28] make: *** [check] Error 1
[01:00:28] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1503725c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0556747b:start=1536975869454016799,finish=1536975869459127538,duration=5110739
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0daa4b5e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; 
