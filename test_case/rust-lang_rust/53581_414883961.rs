plain
[00:45:55] ....................................................................................................
[00:45:58] ....................................................................................................
[00:46:01] ....................................................................................................
[00:46:04] ....................................................................................................
[00:46:07] ..................................................................................F.................
[00:46:13] ....................................................................................................
[00:46:17] ....................................................................................................
[00:46:19] ......................................i.............................................................
[00:46:22] ........................................................................................i.i..ii.....
---
[00:47:04] diff of stderr:
[00:47:04] 
[00:47:04] 2   --> $DIR/issue-46332.rs:19:5
[00:47:04] 3    |
[00:47:04] 4 LL |     TyUInt {};
[00:47:04] -    |     ^^^^^^ did you mean `TyUint`?
[00:47:04] 6 
[00:47:04] 7 error: aborting due to previous error
[00:47:04] 8 
[00:47:04] 
[00:47:04] 
[00:47:04] 
[00:47:04] The actual stderr differed from the expected stderr.
[00:47:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46332/issue-46332.stderr
[00:47:04] To update references, rerun the tests and pass the `--bless` flag
[00:47:04] To only update this specific test, also pass `--test-args issues/issue-46332.rs`
[00:47:04] error: 1 errors occurred comparing output.
[00:47:04] status: exit code: 1
[00:47:04] status: exit code: 1
[00:47:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/u "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:04] 
[00:47:04] 
[00:47:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:04] Build completed unsuccessfully in 0:03:05
[00:47:04] Build completed unsuccessfully in 0:03:05
[00:47:04] Makefile:58: recipe for target 'check' failed
[00:47:04] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cbec385
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0cccfb82:start=1534903907850094846,finish=1534903907858103470,duration=8008624
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f032a19
$ ln -s . checkout && fo
