plain
[00:49:48] ....................................................................................................
[00:49:51] ....................................................................................................
[00:49:54] ............i.......................................................................................
[00:49:57] ....................................................................................................
[00:49:59] .............................................................iiiiiiiii..............................
[00:50:05] ....................................................................................................
[00:50:09] ....................................................................................................
[00:50:12] .........................................i..........................................................
[00:50:15] ...........................................................................................i.i..ii..
---
[00:57:10] ---- [run-pass-valgrind] run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs stdout ----
[00:57:10] 
[00:57:10] error: compilation failed!
[00:57:10] status: exit code: 101
[00:57:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary/auxiliary"
[00:57:10] ------------------------------------------
[00:57:10] 
[00:57:10] ------------------------------------------
[00:57:10] stderr:
[00:57:10] stderr:
[00:57:10] ------------------------------------------
[00:57:10] thread 'main' panicked at 'cannot alloc memory for unsized type', librustc_mir/interpret/place.rs:690:9
[00:57:10] 
[00:57:10] error: internal compiler error: unexpected panic
[00:57:10] 
[00:57:10] note: the compiler unexpectedly panicked. this is a bug.
[00:57:10] note: the compiler unexpectedly panicked. this is a bug.
[00:57:10] 
[00:57:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:10] 
[00:57:10] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:57:10] 
[00:57:10] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:57:10] 
[00:57:10] ------------------------------------------
[00:57:10] 
[00:57:10] thread '[run-pass-valgrind] run-pass-valgrind/unsized-locals/long-live-the-unsized-temporary.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
---
[00:57:10] 
[00:57:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:57:10] 
[00:57:10] 
[00:57:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-valgrind" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-valgrind" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass-valgrind" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-lThu, 30 Aug 2018 15:33:44 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
