plain
[00:47:36] ...i..............................................................................i.................
[00:47:41] ....................................................................................................
[00:47:47] ....................................................................................................
[00:47:53] ....................................................................................................
[00:47:58] ...............i.................iiiiiiiii...................................................
[00:47:58] 
[00:47:58] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:49] ...i..............................................................................i.................
[00:48:54] ....................................................................................................
[00:48:59] ....................................................................................................
[00:49:04] ....................................................................................................
[00:49:09] ...............i.................iiiiiiiii...................................................
[00:49:09] 
[00:49:09]  finished in 71.408
[00:49:09] travis_fold:end:test_ui_nll

---
[00:53:38] .............................................................test [run-pass] run-pass/issue-29227.rs has been running for over 60 seconds
[00:53:49] .......................................
[00:54:14] ....................................................................................................
[00:54:27] ....................................................................................................
[00:54:44] ....................................................F...i...........................................
[00:55:32] .............................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:55:34] .......
[00:55:50] ....................................................................................................
[00:56:30] .......................ii...........................................................i....ii.........
---
[00:58:29] ---- [run-pass] run-pass/mod_dir_path_canonicalized.rs stdout ----
[00:58:29] 
[00:58:29] error: compilation failed!
[00:58:29] status: exit code: 101
[00:58:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/mod_dir_path_canonicalized.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mod_dir_path_canonicalized/auxiliary"
[00:58:29] ------------------------------------------
[00:58:29] 
[00:58:29] ------------------------------------------
[00:58:29] stderr:
---
[00:58:29] error: aborting due to previous error
[00:58:29] 
[00:58:29] For more information about this error, try `rustc --explain E0463`.
[00:58:29] 
[00:58:29] .1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:29] 
[00:58:29] 
[00:58:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:29] Build completed unsuccessfully in 0:13:13
[00:58:29] Build completed unsuccessfully in 0:13:13
[00:58:29] Makefile:58: recipe for target 'check' failed
[00:58:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24be705e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
