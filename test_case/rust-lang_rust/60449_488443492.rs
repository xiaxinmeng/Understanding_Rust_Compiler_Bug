plain
travis_time:end:0d8970bc:start=1556742845856096513,finish=1556742846763756174,duration=907659661
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:43] .................................................................................................... 600/2959
[01:12:58] .................................................................................................... 700/2959
[01:13:09] .................................................................................................... 800/2959
[01:13:18] .................................................................................................... 900/2959
[01:13:33] ...........................................F........................................................ 1000/2959
[01:13:56] .................................................................................................... 1200/2959
[01:14:06] .................................................................................................... 1300/2959
[01:14:18] ..........................ii........................................................................ 1400/2959
[01:14:30] .................................................................................................... 1500/2959
---
[01:18:12] failures:
[01:18:12] 
[01:18:12] ---- [run-pass] run-pass/impl-trait/lifetimes.rs stdout ----
[01:18:12] 
[01:18:12] error: test compilation failed although it shouldn't!
[01:18:12] status: exit code: 1
[01:18:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/impl-trait/lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/lifetimes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/lifetimes/auxiliary"
[01:18:12] ------------------------------------------
[01:18:12] 
[01:18:12] ------------------------------------------
[01:18:12] stderr:
[01:18:12] stderr:
[01:18:12] ------------------------------------------
[01:18:12] error[E0482]: lifetime of return value does not outlive the function call
[01:18:12]   --> /checkout/src/test/run-pass/impl-trait/lifetimes.rs:112:66
[01:18:12]    |
[01:18:12] LL |     fn iter_doesnt_capture_unnecessary_lifetime<'s>(&'s self) -> impl Iterator<Item = &'s u8> {
[01:18:12]    |
[01:18:12] note: the return value is only valid for the lifetime 'unnecessary_lifetime as defined on the impl at 111:6
[01:18:12]   --> /checkout/src/test/run-pass/impl-trait/lifetimes.rs:111:6
[01:18:12]    |
[01:18:12]    |
[01:18:12] LL | impl<'unnecessary_lifetime> MyVec {
[01:18:12] 
[01:18:12] error: aborting due to previous error
[01:18:12] 
[01:18:12] 
---
[01:18:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:18:12] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:12] 
[01:18:12] 
[01:18:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:12] 
[01:18:12] 
[01:18:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:12] Build completed unsuccessfully in 0:10:52
[01:18:12] Build completed unsuccessfully in 0:10:52
[01:18:12] Makefile:48: recipe for target 'check' failed
[01:18:12] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:106250d9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 21:52:31 UTC 2019
