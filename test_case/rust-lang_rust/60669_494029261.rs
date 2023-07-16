plain
travis_time:end:0a91befa:start=1558360264583742863,finish=1558360267041073851,duration=2457330988
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:17:41] .................................................................................................... 1600/5537
[01:17:45] ..............................i..................................................................... 1700/5537
[01:17:49] .................................................................................................... 1800/5537
[01:17:53] .................................................................................................... 1900/5537
[01:17:57] .......................................F............................................................ 2000/5537
[01:18:05] .................................................................................................... 2200/5537
[01:18:09] .................................................................................................... 2300/5537
[01:18:13] .................................................................................................... 2400/5537
[01:18:18] .................................................................................................... 2500/5537
[01:18:18] .................................................................................................... 2500/5537
[01:18:22] .................................................................................................... 2600/5537
[01:18:26] .................................................................................................... 2700/5537
[01:18:31] .................................................................................................... 2800/5537
[01:18:35] .................................................................................................... 2900/5537
[01:18:40] .................................................................................................... 3000/5537
[01:18:43] .................................................................................................... 3100/5537
[01:18:47] .....................................F.............................................................. 3200/5537
[01:18:56] .........i.......................................................................................... 3400/5537
[01:19:00] ...................................................................................ii...i..ii....... 3500/5537
[01:19:04] .................................................................................................... 3600/5537
[01:19:08] .................................................................................................... 3700/5537
---
[01:20:24] 
[01:20:24] 1 error: lifetimes cannot use keyword names
[01:20:24] -   --> $DIR/issue-10412.rs:1:20
[01:20:24] -    |
[01:20:24] - LL | trait Serializable<'self, T> {
[01:20:24] - 
[01:20:24] - 
[01:20:24] - error: lifetimes cannot use keyword names
[01:20:24] 9    |
[01:20:24] 9    |
[01:20:24] 10 LL |     fn serialize(val : &'self T) -> Vec<u8>;
[01:20:24] 15    |
[01:20:24] 15    |
[01:20:24] 16 LL |     fn deserialize(repr : &[u8]) -> &'self T;
[01:20:24] + 
[01:20:24] + 
[01:20:24] + error: lifetimes cannot use keyword names
[01:20:24] +    |
[01:20:24] +    |
[01:20:24] + LL | trait Serializable<'self, T> {
[01:20:24] 18 
[01:20:24] 19 error: lifetimes cannot use keyword names
[01:20:24] 20   --> $DIR/issue-10412.rs:6:6
[01:20:24] 
[01:20:24] 
[01:20:24] 
[01:20:24] The actual stderr differed from the expected stderr.
[01:20:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412/issue-10412.stderr
[01:20:24] To update references, rerun the tests and pass the `--bless` flag
[01:20:24] To only update this specific test, also pass `--test-args issues/issue-10412.rs`
[01:20:24] error: 1 errors occurred comparing output.
[01:20:24] status: exit code: 1
[01:20:24] status: exit code: 1
[01:20:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10412.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412/auxiliary" "-A" "unused"
[01:20:24] ------------------------------------------
[01:20:24] 
[01:20:24] ------------------------------------------
[01:20:24] stderr:
[01:20:24] stderr:
[01:20:24] ------------------------------------------
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:2:25
[01:20:24]    |
[01:20:24] LL |     fn serialize(val : &'self T) -> Vec<u8>; //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:3:38
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL |     fn deserialize(repr : &[u8]) -> &'self T; //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:1:20
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | trait Serializable<'self, T> { //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:6:6
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:6:36
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:10:25
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL |     fn serialize(val : &'self str) -> Vec<u8> { //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:13:37
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL |     fn deserialize(repr: &[u8]) -> &'self str { //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] 
[01:20:24] error[E0726]: implicit elided lifetime not allowed here
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
[01:20:24]    |             ^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Serializable<'_, str>`
[01:20:24] error[E0277]: the size for values of type `str` cannot be known at compilation time
[01:20:24]   --> /checkout/src/test/ui/issues/issue-10412.rs:6:13
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
[01:20:24]    |
[01:20:24]    = help: the trait `std::marker::Sized` is not implemented for `str`
[01:20:24]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:20:24] 
---
[01:20:24] 1 error: lifetimes cannot use keyword names
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:3:8
[01:20:24] +   --> $DIR/lifetime-no-keyword.rs:3:18
[01:20:24] 3    |
[01:20:24] 4 LL | fn baz<'let>(a: &'let isize) { }
[01:20:24] +    |                  ^^^^
[01:20:24] 6 
[01:20:24] 7 error: lifetimes cannot use keyword names
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:3:18
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:3:18
[01:20:24] +   --> $DIR/lifetime-no-keyword.rs:3:8
[01:20:24] 9    |
[01:20:24] 10 LL | fn baz<'let>(a: &'let isize) { }
[01:20:24] +    |        ^^^^
[01:20:24] 12 
[01:20:24] 13 error: lifetimes cannot use keyword names
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:5:8
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:5:8
[01:20:24] +   --> $DIR/lifetime-no-keyword.rs:5:19
[01:20:24] 15    |
[01:20:24] 16 LL | fn zab<'self>(a: &'self isize) { }
[01:20:24] +    |                   ^^^^^
[01:20:24] 18 
[01:20:24] 19 error: lifetimes cannot use keyword names
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:5:19
[01:20:24] -   --> $DIR/lifetime-no-keyword.rs:5:19
[01:20:24] +   --> $DIR/lifetime-no-keyword.rs:5:8
[01:20:24] 21    |
[01:20:24] 22 LL | fn zab<'self>(a: &'self isize) { }
[01:20:24] +    |        ^^^^^
[01:20:24] 24 
[01:20:24] 25 error: aborting due to 4 previous errors
[01:20:24] 26 
[01:20:24] 26 
[01:20:24] 
[01:20:24] 
[01:20:24] The actual stderr differed from the expected stderr.
[01:20:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-no-keyword/lifetime-no-keyword.stderr
[01:20:24] To update references, rerun the tests and pass the `--bless` flag
[01:20:24] To only update this specific test, also pass `--test-args lifetimes/lifetime-no-keyword.rs`
[01:20:24] error: 1 errors occurred comparing output.
[01:20:24] status: exit code: 1
[01:20:24] status: exit code: 1
[01:20:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-no-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-no-keyword" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-no-keyword/auxiliary" "-A" "unused"
[01:20:24] ------------------------------------------
[01:20:24] 
[01:20:24] ------------------------------------------
[01:20:24] stderr:
[01:20:24] stderr:
[01:20:24] ------------------------------------------
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/lifetimes/lifetime-no-keyword.rs:3:18
[01:20:24]    |
[01:20:24] LL | fn baz<'let>(a: &'let isize) { } //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/lifetimes/lifetime-no-keyword.rs:3:8
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | fn baz<'let>(a: &'let isize) { } //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/lifetimes/lifetime-no-keyword.rs:5:19
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | fn zab<'self>(a: &'self isize) { } //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: lifetimes cannot use keyword names
[01:20:24]   --> /checkout/src/test/ui/lifetimes/lifetime-no-keyword.rs:5:8
[01:20:24]    |
[01:20:24]    |
[01:20:24] LL | fn zab<'self>(a: &'self isize) { } //~ ERROR lifetimes cannot use keyword names
[01:20:24] 
[01:20:24] error: aborting due to 4 previous errors
[01:20:24] 
[01:20:24] 
---
[01:20:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:20:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:20:24] 
[01:20:24] 
[01:20:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:24] 
[01:20:24] 
[01:20:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:24] Build completed unsuccessfully in 0:05:04
[01:20:24] Build completed unsuccessfully in 0:05:04
[01:20:24] Makefile:48: recipe for target 'check' failed
[01:20:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ef0238
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 20 15:11:42 UTC 2019
---
travis_time:end:0912bcaf:start=1558365103774370391,finish=1558365103779912494,duration=5542103
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1104aa5e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fdce4b6
travis_time:start:0fdce4b6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21856b84
$ dmesg | grep -i kill
