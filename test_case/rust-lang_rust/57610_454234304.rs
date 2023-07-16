plain
travis_time:end:15f2dc7f:start=1547511467079056177,finish=1547511469206791470,duration=2127735293
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:12:40] .................................................................................................... 2500/5303
[01:12:44] .................................................................................................... 2600/5303
[01:12:48] .................................................................................................... 2700/5303
[01:12:54] .................................................................................................... 2800/5303
[01:12:58] .......F............................................................................................ 2900/5303
[01:13:06] .................................................................................................... 3100/5303
[01:13:10] ....................................................................................i............... 3200/5303
[01:13:14] .................................................................................................... 3300/5303
[01:13:18] ..............................................ii...i..ii............................................ 3400/5303
---
[01:14:39] 
[01:14:39] ---- [ui] ui/issues/issue-5067.rs stdout ----
[01:14:39] diff of stderr:
[01:14:39] 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $()* ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $()+ ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( [$()*] ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( [$()+] ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $($()* $(),* $(a)* $(a),* )* ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $($()* $(),* $(a)* $(a),* )+ ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $(a $()+)* ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $(a $()*)+ ) => {};
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     (a $e1:expr $($(, a $e2:expr)*)*) => ([$e1 $($(, $e2)*)*]);
[01:14:39] - 
[01:14:39] - 
[01:14:39] - error: repetition matches empty token tree
[01:14:39] -    |
[01:14:39] -    |
[01:14:39] - LL |     ( $()* ) => {}
[01:14:39] - 
[01:14:39] - error: aborting due to 10 previous errors
[01:14:39] - 
[01:14:39] - 
[01:14:39] - 
[01:14:39] 
[01:14:39] 
[01:14:39] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5067/issue-5067.stderr`: No such file or directory (os error 2)
[01:14:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:39] 
[01:14:39] 
[01:14:39] failures:
[01:14:39] failures:
[01:14:39]     [ui] ui/issues/issue-5067.rs
[01:14:39] 
[01:14:39] test result: FAILED. 5279 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
[01:14:39] 
[01:14:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:14:39] 
[01:14:39] 
[01:14:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:39] 
[01:14:39] 
[01:14:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:39] Build completed unsuccessfully in 0:04:49
[01:14:39] Build completed unsuccessfully in 0:04:49
[01:14:39] make: *** [check] Error 1
[01:14:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09f07e98
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 01:32:39 UTC 2019
