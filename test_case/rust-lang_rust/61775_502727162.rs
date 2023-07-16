plain
travis_time:end:084e7edc:start=1560781049601578949,finish=1560781051989292389,duration=2387713440
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:56:43] .................................................................................................... 1400/5693
[00:56:46] .................................................................................................... 1500/5693
[00:56:49] .................................................................................................... 1600/5693
[00:56:51] .................................................................................................... 1700/5693
[00:56:55] ..i................................................................................................. 1800/5693
[00:56:58] ...........................................................................F........................ 1900/5693
[00:57:06] .................................................................................................... 2100/5693
[00:57:09] ........................................................i........................................... 2200/5693
[00:57:13] .................................................................................................... 2300/5693
[00:57:17] .................................................................................................... 2400/5693
---
[00:59:28] 1 error: lifetime may not live long enough
[00:59:28] -   --> $DIR/error-handling.rs:10:1
[00:59:28] +   --> $DIR/error-handling.rs:13:56
[00:59:28] 3    |
[00:59:28] - LL | existential type E<'a, 'b>: Sized;
[00:59:28] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
[00:59:28] - ...
[00:59:28] 7 LL | fn foo<'a, 'b, 'c>(x: &'static i32, mut y: &'a i32) -> E<'b, 'c> {
[00:59:28] -    |        -- lifetime `'a` defined here
[00:59:28] +    |        -- lifetime `'a` defined here                   ^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
[00:59:28] 9 help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a constraint
[00:59:28] 10    |
[00:59:28] 11 LL | existential type E<'a, 'b>: Sized; + 'a
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling/error-handling.stderr
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling/error-handling.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args impl-trait/multiple-lifetimes/error-handling.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/multiple-lifetimes/error-handling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
[00:59:28] stderr:
[00:59:28] ------------------------------------------
[00:59:28] error: lifetime may not live long enough
[00:59:28]   --> /checkout/src/test/ui/impl-trait/multiple-lifetimes/error-handling.rs:13:56
[00:59:28]    |
[00:59:28] LL | fn foo<'a, 'b, 'c>(x: &'static i32, mut y: &'a i32) -> E<'b, 'c> {
[00:59:28]    |        -- lifetime `'a` defined here                   ^^^^^^^^^ opaque type requires that `'a` must outlive `'static`
[00:59:28] help: to allow this `impl Trait` to capture borrowed data with lifetime `'a`, add `'a` as a constraint
[00:59:28]    |
[00:59:28] LL | existential type E<'a, 'b>: Sized; + 'a
[00:59:28] 
[00:59:28] error: aborting due to previous error
[00:59:28] 
[00:59:28] 
---
[00:59:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:28] 
[00:59:28] 
[00:59:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:28] 
[00:59:28] 
[00:59:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:28] Build completed unsuccessfully in 0:54:44
---
travis_time:end:01056e7d:start=1560784632740936657,finish=1560784632745644949,duration=4708292
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:097f87a2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:170bce05
travis_time:start:170bce05
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0365ad1d
$ dmesg | grep -i kill
