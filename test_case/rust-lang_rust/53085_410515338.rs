plain
[00:44:58] ....................................................................................................
[00:45:01] ....................................................................................................
[00:45:03] ....................................................................................................
[00:45:06] ....................................................................................................
[00:45:09] .iiiiiiiii..........................................................................................
[00:45:15] ....................................................................................................
[00:45:18] .....i..............................................................................................
[00:45:21] .........i..........................................................................................
[00:45:24] ....................................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:34] 
[00:53:34] running 96 tests
[00:55:22] ...................F..........................F............................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
--------------
[00:56:34] error[E0432]: unresolved import `syntax::util::ThinVec`
[00:56:34]    |
[00:56:34]    |
[00:56:34] 42 | use syntax::util::ThinVec;
[00:56:34]    |     ^^^^^^^^^^^^^^^^^^^^^ no `ThinVec` in `util`
[00:56:34] error: aborting due to previous error
[00:56:34] 
[00:56:34] For more information about this error, try `rustc --explain E0432`.
[00:56:34] 
---
[00:56:34] test result: FAILED. 94 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:34] 
[00:56:34] 
[00:56:34] 
[00:56:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--ho
