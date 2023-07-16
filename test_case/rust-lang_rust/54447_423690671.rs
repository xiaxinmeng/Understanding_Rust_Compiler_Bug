plain
[00:57:56] ....................................................................................................
[00:57:59] .....................................................i..............................................
[00:58:02] ....................................................................................................
[00:58:05] ....................................................................................................
[00:58:08] .iiiiiiiii..........................................................................................
[00:58:14] ....................................................................................................
[00:58:17] .....................................................................................i..............
[00:58:20] ....................................................................................................
[00:58:23] ........................................i.i..ii.....................................................
---
[01:04:03] ........................................i...........................................................
[01:04:15] ....................................................................................................
[01:04:30] ....................................................................................................
[01:04:40] ....................................................................................................
[01:04:57] ................i......................................................F............................
[01:05:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:05:03] failures:
[01:05:03] 
[01:05:03] ---- [run-pass] run-pass/type-ascription.rs stdout ----
[01:05:03] 
[01:05:03] 
[01:05:03] error: compilation failed!
[01:05:03] status: signal: 6
[01:05:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/type-ascription.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/type-ascription/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/type-ascription/auxiliary"
[01:05:03] ------------------------------------------
[01:05:03] 
[01:05:03] ------------------------------------------
[01:05:03] stderr:
[01:05:03] stderr:
[01:05:03] ------------------------------------------
[01:05:03] 
[01:05:03] thread 'main' has overflowed its stack
[01:" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:03] 
[01:05:03] 
[01:05:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:03] Build completed unsuccessfully in 0:16:10
[01:05:03] Build completed unsuccessfully in 0:16:10
[01:05:03] make: *** [check] Error 1
[01:05:03] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:058a5bd8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0f4959d5:start=1537570157538359815,finish=1537570157542322330,duration=3962515
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a8a7aa0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; t
