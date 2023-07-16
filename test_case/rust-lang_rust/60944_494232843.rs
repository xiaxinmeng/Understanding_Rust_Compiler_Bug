plain
travis_time:end:0f2ed0e0:start=1558407553949009403,finish=1558407642786432471,duration=88837423068
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:37] .................................................................................................... 4400/5534
[01:13:40] .................................................................................................... 4500/5534
[01:13:44] .................................................................................................... 4600/5534
[01:13:48] .................................................................................................... 4700/5534
[01:13:54] .........................................................................................F.......... 4800/5534
[01:14:01] .................................................................................................... 5000/5534
[01:14:05] .................................................................................................... 5100/5534
[01:14:08] .................................................................................................... 5200/5534
[01:14:12] .................................................................................................... 5300/5534
---
[01:14:19] normalized stderr:
[01:14:19] error: cannot infer an appropriate lifetime
[01:14:19]   --> $DIR/arbitrary_self_types_impl_trait_lifetime.rs:21:9
[01:14:19]    |
[01:14:19] LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug {
[01:14:19]    |                            -------------------- this return type evaluates to the `'static` lifetime...
[01:14:19]    |         ^^^^ ...but this borrow...
[01:14:19]    |
[01:14:19]    |
[01:14:19] note: ...can't outlive the anonymous lifetime #1 defined on the method body at 20:5
[01:14:19]   --> $DIR/arbitrary_self_types_impl_trait_lifetime.rs:20:5
[01:14:19]    |
[01:14:19] LL | /     fn f(self: Bar<'_>) -> impl std::fmt::Debug {
[01:14:19] LL | |
[01:14:19] LL | |     }
[01:14:19]    | |_____^
[01:14:19]    | |_____^
[01:14:19] help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 20:5
[01:14:19]    |
[01:14:19] LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug + '_ {
[01:14:19] 
[01:14:19] error: aborting due to previous error
[01:14:19] 
[01:14:19] 
[01:14:19] 
[01:14:19] 
[01:14:19] 
[01:14:19] The actual stderr differed from the expected stderr.
[01:14:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_impl_trait_lifetime/arbitrary_self_types_impl_trait_lifetime.stderr
[01:14:19] To update references, rerun the tests and pass the `--bless` flag
[01:14:19] To only update this specific test, also pass `--test-args self/arbitrary_self_types_impl_trait_lifetime.rs`
[01:14:19] error: 1 errors occurred comparing output.
[01:14:19] status: exit code: 1
[01:14:19] status: exit code: 1
[01:14:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_impl_trait_lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_impl_trait_lifetime" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_impl_trait_lifetime/auxiliary" "-A" "unused"
[01:14:19] ------------------------------------------
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] stderr:
[01:14:19] stderr:
[01:14:19] ------------------------------------------
[01:14:19] error: cannot infer an appropriate lifetime
[01:14:19]   --> /checkout/src/test/ui/self/arbitrary_self_types_impl_trait_lifetime.rs:21:9
[01:14:19]    |
[01:14:19] LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug {
[01:14:19]    |                            -------------------- this return type evaluates to the `'static` lifetime...
[01:14:19]    |         ^^^^ ...but this borrow...
[01:14:19]    |
[01:14:19]    |
[01:14:19] note: ...can't outlive the anonymous lifetime #1 defined on the method body at 20:5
[01:14:19]    |
[01:14:19]    |
[01:14:19] LL | /     fn f(self: Bar<'_>) -> impl std::fmt::Debug {
[01:14:19] LL | |         //~^ ERROR cannot infer an appropriate lifetime
[01:14:19] LL | |     }
[01:14:19]    | |_____^
[01:14:19]    | |_____^
[01:14:19] help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 20:5
[01:14:19]    |
[01:14:19] LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug + '_ {
[01:14:19] 
[01:14:19] error: aborting due to previous error
[01:14:19] 
[01:14:19] 
---
[01:14:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:14:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:19] 
[01:14:19] 
[01:14:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:19] 
[01:14:19] 
[01:14:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:19] Build completed unsuccessfully in 0:04:37
[01:14:19] Build completed unsuccessfully in 0:04:37
[01:14:19] Makefile:48: recipe for target 'check' failed
[01:14:19] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c9b81a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 21 04:15:11 UTC 2019
---
travis_time:end:0a52291a:start=1558412113350049912,finish=1558412113357189671,duration=7139759
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09191202
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13d40990
$ dmesg | grep -i kill
