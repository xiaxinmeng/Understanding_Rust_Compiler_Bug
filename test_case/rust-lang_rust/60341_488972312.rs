plain
travis_time:end:072b724e:start=1556862230604515911,finish=1556862317054375468,duration=86449859557
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:09] .................................................................................................... 2400/5483
[01:11:14] .................................................................................................... 2500/5483
[01:11:18] .................................................................................................... 2600/5483
[01:11:21] .................................................................................................... 2700/5483
[01:11:26] .............F...................................................................................... 2800/5483
[01:11:34] .................................................................................................... 3000/5483
[01:11:37] .................................................................................................... 3100/5483
[01:11:40] .................................................................................................... 3200/5483
[01:11:44] ...............................................................................i.................... 3300/5483
---
[01:13:05] 
[01:13:05] - error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[01:13:05] -   --> $DIR/issue-43733.rs:18:5
[01:13:05] -    |
[01:13:05] - LL |     __KEY.get(init)
[01:13:05] -    |     ^^^^^^^^^^^^^^^ call to unsafe function
[01:13:05] -    |
[01:13:05] -    = note: consult the function's documentation for information on how to avoid undefined behavior
[01:13:05] - error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[01:13:05] + error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[01:13:05] 10   --> $DIR/issue-43733.rs:22:5
[01:13:05] 11    |
[01:13:05] 11    |
[01:13:05] 12 LL |     std::thread::LocalKey::new(__getit, Default::default);
[01:13:05] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[01:13:05] -    |
[01:13:05] -    |
[01:13:05] -    = note: consult the function's documentation for information on how to avoid undefined behavior
[01:13:05] 16 
[01:13:05] - error: aborting due to 2 previous errors
[01:13:05] + error: aborting due to previous error
[01:13:05] 18 
[01:13:05] 18 
[01:13:05] - For more information about this error, try `rustc --explain E0133`.
[01:13:05] + For more information about this error, try `rustc --explain E0061`.
[01:13:05] 20 
[01:13:05] 
[01:13:05] 
[01:13:05] The actual stderr differed from the expected stderr.
[01:13:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733/issue-43733.stderr
[01:13:05] To update references, rerun the tests and pass the `--bless` flag
[01:13:05] To only update this specific test, also pass `--test-args issues/issue-43733.rs`
[01:13:05] error: 1 errors occurred comparing output.
[01:13:05] status: exit code: 1
[01:13:05] status: exit code: 1
[01:13:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43733/auxiliary" "-A" "unused"
[01:13:05] ------------------------------------------
[01:13:05] 
[01:13:05] ------------------------------------------
[01:13:05] stderr:
[01:13:05] stderr:
[01:13:05] ------------------------------------------
[01:13:05] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[01:13:05]   --> /checkout/src/test/ui/issues/issue-43733.rs:22:5
[01:13:05]    |
[01:13:05] LL |     std::thread::LocalKey::new(__getit, Default::default);
[01:13:05] 
[01:13:05] error: aborting due to previous error
[01:13:05] 
[01:13:05] For more information about this error, try `rustc --explain E0061`.
---
[01:13:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:13:05] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:05] 
[01:13:05] 
[01:13:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:05] 
[01:13:05] 
[01:13:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:05] Build completed unsuccessfully in 0:04:17
[01:13:05] Build completed unsuccessfully in 0:04:17
[01:13:05] Makefile:48: recipe for target 'check' failed
[01:13:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f843bee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May  3 06:58:32 UTC 2019
