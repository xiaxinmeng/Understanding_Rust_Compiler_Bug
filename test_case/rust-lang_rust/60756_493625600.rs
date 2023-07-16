plain
travis_time:end:05399439:start=1558130585300147390,finish=1558130672456236753,duration=87156089363
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:15:06] .................................................................................................... 1400/5531
[01:15:09] .................................................................................................... 1500/5531
[01:15:12] .................................................................................................... 1600/5531
[01:15:16] .............................i...................................................................... 1700/5531
[01:15:19] ......................................................................................F............. 1800/5531
[01:15:27] .................................................................................................... 2000/5531
[01:15:30] ..........................................................................i......................... 2100/5531
[01:15:34] .................................................................................................... 2200/5531
[01:15:38] .................................................................................................... 2300/5531
---
[01:17:47] 
[01:17:47] ---- [ui] ui/impl-trait/hidden-lifetimes.rs stdout ----
[01:17:47] diff of stderr:
[01:17:47] 
[01:17:47] 1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
[01:17:47] -   --> $DIR/hidden-lifetimes.rs:23:54
[01:17:47] +   --> $DIR/hidden-lifetimes.rs:28:54
[01:17:47] 3    |
[01:17:47] 4 LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a {
[01:17:47] 
[01:17:47] 6    |
[01:17:47] 6    |
[01:17:47] - note: hidden type `&'a mut &'b T` captures the lifetime 'b as defined on the function body at 23:17
[01:17:47] -   --> $DIR/hidden-lifetimes.rs:23:17
[01:17:47] + note: hidden type `&'a mut &'b T` captures the lifetime 'b as defined on the function body at 28:17
[01:17:47] +   --> $DIR/hidden-lifetimes.rs:28:17
[01:17:47] 9    |
[01:17:47] 10 LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a {
[01:17:47] 
[01:17:47] 12 
[01:17:47] 12 
[01:17:47] 13 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
[01:17:47] -   --> $DIR/hidden-lifetimes.rs:35:70
[01:17:47] +   --> $DIR/hidden-lifetimes.rs:45:70
[01:17:47] 15    |
[01:17:47] 16 LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a {
[01:17:47] 
[01:17:47] 18    |
[01:17:47] 18    |
[01:17:47] - note: hidden type `std::rc::Rc<std::cell::RefCell<&'b T>>` captures the lifetime 'b as defined on the function body at 35:24
[01:17:47] -   --> $DIR/hidden-lifetimes.rs:35:24
[01:17:47] + note: hidden type `std::rc::Rc<std::cell::RefCell<&'b T>>` captures the lifetime 'b as defined on the function body at 45:24
[01:17:47] +   --> $DIR/hidden-lifetimes.rs:45:24
[01:17:47] 21    |
[01:17:47] 22 LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a {
[01:17:47] 
[01:17:47] 
[01:17:47] The actual stderr differed from the expected stderr.
[01:17:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes/hidden-lifetimes.stderr
[01:17:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes/hidden-lifetimes.stderr
[01:17:47] To update references, rerun the tests and pass the `--bless` flag
[01:17:47] To only update this specific test, also pass `--test-args impl-trait/hidden-lifetimes.rs`
[01:17:47] error: 1 errors occurred comparing output.
[01:17:47] status: exit code: 1
[01:17:47] status: exit code: 1
[01:17:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/hidden-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes/auxiliary" "-A" "unused"
[01:17:47] ------------------------------------------
[01:17:47] 
[01:17:47] ------------------------------------------
[01:17:47] stderr:
[01:17:47] stderr:
[01:17:47] ------------------------------------------
[01:17:47] error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
[01:17:47]   --> /checkout/src/test/ui/impl-trait/hidden-lifetimes.rs:28:54
[01:17:47]    |
[01:17:47] LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a {
[01:17:47]    |
[01:17:47]    |
[01:17:47] note: hidden type `&'a mut &'b T` captures the lifetime 'b as defined on the function body at 28:17
[01:17:47]   --> /checkout/src/test/ui/impl-trait/hidden-lifetimes.rs:28:17
[01:17:47]    |
[01:17:47] LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a {
[01:17:47] 
[01:17:47] 
[01:17:47] error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
[01:17:47]   --> /checkout/src/test/ui/impl-trait/hidden-lifetimes.rs:45:70
[01:17:47]    |
[01:17:47] LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a {
[01:17:47]    |
[01:17:47]    |
[01:17:47] note: hidden type `std::rc::Rc<std::cell::RefCell<&'b T>>` captures the lifetime 'b as defined on the function body at 45:24
[01:17:47]   --> /checkout/src/test/ui/impl-trait/hidden-lifetimes.rs:45:24
[01:17:47]    |
[01:17:47] LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a {
[01:17:47] 
[01:17:47] error: aborting due to 2 previous errors
[01:17:47] 
[01:17:47] For more information about this error, try `rustc --explain E0700`.
---
[01:17:47] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:17:47] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:47] 
[01:17:47] 
[01:17:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:47] 
[01:17:47] 
[01:17:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:47] Build completed unsuccessfully in 0:04:52
[01:17:47] Build completed unsuccessfully in 0:04:52
[01:17:47] make: *** [check] Error 1
[01:17:47] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05b0cb48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 23:22:29 UTC 2019
---
travis_time:end:0e03ab1c:start=1558135350289765692,finish=1558135350294189820,duration=4424128
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b2ab3d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:037c34c7
travis_time:start:037c34c7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00f50919
$ dmesg | grep -i kill
