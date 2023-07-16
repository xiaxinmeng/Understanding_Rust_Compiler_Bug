plain
travis_time:end:043dc8e6:start=1558370374253591278,finish=1558370461491867692,duration=87238276414
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:23] .................................................................................................... 4400/5533
[01:12:26] .................................................................................................... 4500/5533
[01:12:30] .................................................................................................... 4600/5533
[01:12:34] .................................................................................................... 4700/5533
[01:12:41] ...........................................................................................F........ 4800/5533
[01:12:47] .................................................................................................... 5000/5533
[01:12:52] .................................................................................................... 5100/5533
[01:12:56] .................................................................................................... 5200/5533
[01:12:59] .................................................................................................... 5300/5533
---
[01:13:06] 
[01:13:06] ---- [ui] ui/self/arbitrary_self_types_lifetime.rs stdout ----
[01:13:06] diff of stderr:
[01:13:06] 
[01:13:06] 19 LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug + '_ {
[01:13:06] 21 
[01:13:06] - error: aborting due to previous error
[01:13:06] + error[E0034]: multiple applicable items in scope
[01:13:06] +   --> $DIR/arbitrary_self_types_lifetime.rs:73:17
[01:13:06] +   --> $DIR/arbitrary_self_types_lifetime.rs:73:17
[01:13:06] +    |
[01:13:06] + LL |     { Bar(&foo).c() };
[01:13:06] +    |                 ^ multiple `c` found
[01:13:06] +    |
[01:13:06] + note: candidate #1 is defined in an impl for the type `Foo`
[01:13:06] +   --> $DIR/arbitrary_self_types_lifetime.rs:26:5
[01:13:06] +    |
[01:13:06] + LL |     fn c(self: Bar<'_>) -> Bar<'_> {
[01:13:06] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:13:06] + note: candidate #2 is defined in an impl for the type `Bar<'_>`
[01:13:06] +   --> $DIR/arbitrary_self_types_lifetime.rs:51:5
[01:13:06] +    |
[01:13:06] + LL |     fn c(self: Bar<'a>, f: &Foo) -> (Self, &Foo) { (self, f) }
[01:13:06] 23 
[01:13:06] + error: aborting due to 2 previous errors
[01:13:06] + 
[01:13:06] + For more information about this error, try `rustc --explain E0034`.
[01:13:06] + For more information about this error, try `rustc --explain E0034`.
[01:13:06] 24 
[01:13:06] 
[01:13:06] 
[01:13:06] The actual stderr differed from the expected stderr.
[01:13:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_lifetime/arbitrary_self_types_lifetime.stderr
[01:13:06] To update references, rerun the tests and pass the `--bless` flag
[01:13:06] To only update this specific test, also pass `--test-args self/arbitrary_self_types_lifetime.rs`
[01:13:06] error: 1 errors occurred comparing output.
[01:13:06] status: exit code: 1
[01:13:06] status: exit code: 1
[01:13:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_lifetime" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_lifetime/auxiliary" "-A" "unused"
[01:13:06] ------------------------------------------
[01:13:06] 
[01:13:06] ------------------------------------------
[01:13:06] stderr:
[01:13:06] stderr:
[01:13:06] ------------------------------------------
[01:13:06] error: cannot infer an appropriate lifetime
[01:13:06]   --> /checkout/src/test/ui/self/arbitrary_self_types_lifetime.rs:39:9
[01:13:06]    |
[01:13:06] LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug {
[01:13:06]    |                            -------------------- this return type evaluates to the `'static` lifetime...
[01:13:06]    |         ^^^^ ...but this borrow...
[01:13:06]    |
[01:13:06]    |
[01:13:06] note: ...can't outlive the anonymous lifetime #1 defined on the method body at 38:5
[01:13:06]   --> /checkout/src/test/ui/self/arbitrary_self_types_lifetime.rs:38:5
[01:13:06]    |
[01:13:06] LL | /     fn f(self: Bar<'_>) -> impl std::fmt::Debug {
[01:13:06] LL | |         //~^ ERROR cannot infer an appropriate lifetime
[01:13:06] LL | |     }
[01:13:06]    | |_____^
[01:13:06]    | |_____^
[01:13:06] help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 38:5
[01:13:06]    |
[01:13:06] LL |     fn f(self: Bar<'_>) -> impl std::fmt::Debug + '_ {
[01:13:06] 
[01:13:06] error[E0034]: multiple applicable items in scope
[01:13:06]   --> /checkout/src/test/ui/self/arbitrary_self_types_lifetime.rs:73:17
[01:13:06]    |
[01:13:06]    |
[01:13:06] LL |     { Bar(&foo).c() };
[01:13:06]    |                 ^ multiple `c` found
[01:13:06]    |
[01:13:06] note: candidate #1 is defined in an impl for the type `Foo`
[01:13:06]   --> /checkout/src/test/ui/self/arbitrary_self_types_lifetime.rs:26:5
[01:13:06]    |
[01:13:06] LL |     fn c(self: Bar<'_>) -> Bar<'_> {
[01:13:06]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:13:06] note: candidate #2 is defined in an impl for the type `Bar<'_>`
[01:13:06]   --> /checkout/src/test/ui/self/arbitrary_self_types_lifetime.rs:51:5
[01:13:06]    |
[01:13:06] LL |     fn c(self: Bar<'a>, f: &Foo) -> (Self, &Foo) { (self, f) }
[01:13:06] 
[01:13:06] error: aborting due to 2 previous errors
[01:13:06] 
[01:13:06] For more information about this error, try `rustc --explain E0034`.
---
[01:13:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:13:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:06] 
[01:13:06] 
[01:13:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:06] 
[01:13:06] 
[01:13:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:06] Build completed unsuccessfully in 0:04:48
[01:13:06] Build completed unsuccessfully in 0:04:48
[01:13:06] make: *** [check] Error 1
[01:13:06] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20fb96cf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 20 17:54:17 UTC 2019
