plain
travis_time:end:2018a6fb:start=1555944708029859657,finish=1555944803482784570,duration=95452924913
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:08] .................................................................................................... 700/5549
[01:03:13] .................................................................................................... 800/5549
[01:03:17] .................................................................................................... 900/5549
[01:03:22] ............i...............i....................................................................... 1000/5549
[01:03:25] .............................................iiiii.................................................. 1100/5549
[01:03:29] ..............F..................................................................................... 1200/5549
[01:03:33] .................................................................................................... 1400/5549
[01:03:37] .................................................................................................... 1500/5549
[01:03:39] .................................................................................................... 1600/5549
[01:03:42] .............................................................i...................................... 1700/5549
---
[01:06:07] 
[01:06:07] ---- [ui] ui/e0119/conflict-with-std.rs stdout ----
[01:06:07] diff of stderr:
[01:06:07] 
[01:06:07] 1 error[E0119]: conflicting implementations of trait `std::convert::AsRef<Q>` for type `std::boxed::Box<Q>`:
[01:06:07] +   --> $DIR/conflict-with-std.rs:5:1
[01:06:07] 3    |
[01:06:07] 3    |
[01:06:07] 4 LL | impl AsRef<Q> for Box<Q> {
[01:06:07] 
[01:06:07] 
[01:06:07] 9              where T: ?Sized;
[01:06:07] 10 
[01:06:07] 11 error[E0119]: conflicting implementations of trait `std::convert::From<S>` for type `S`:
[01:06:07] +   --> $DIR/conflict-with-std.rs:12:1
[01:06:07] 13    |
[01:06:07] 13    |
[01:06:07] 14 LL | impl From<S> for S {
[01:06:07] 
[01:06:07] 
[01:06:07] 18            - impl<T> std::convert::From<T> for T;
[01:06:07] 19 
[01:06:07] 20 error[E0119]: conflicting implementations of trait `std::convert::TryFrom<X>` for type `X`:
[01:06:07] +   --> $DIR/conflict-with-std.rs:19:1
[01:06:07] 22    |
[01:06:07] 22    |
[01:06:07] 23 LL | impl TryFrom<X> for X {
[01:06:07] 
[01:06:07] 
[01:06:07] The actual stderr differed from the expected stderr.
[01:06:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/conflict-with-std.stderr
[01:06:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/conflict-with-std.stderr
[01:06:07] To update references, rerun the tests and pass the `--bless` flag
[01:06:07] To only update this specific test, also pass `--test-args e0119/conflict-with-std.rs`
[01:06:07] error: 1 errors occurred comparing output.
[01:06:07] status: exit code: 1
[01:06:07] status: exit code: 1
[01:06:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/e0119/conflict-with-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/auxiliary" "-A" "unused"
[01:06:07] ------------------------------------------
[01:06:07] 
[01:06:07] ------------------------------------------
[01:06:07] stderr:
[01:06:07] stderr:
[01:06:07] ------------------------------------------
[01:06:07] error[E0119]: conflicting implementations of trait `std::convert::AsRef<Q>` for type `std::boxed::Box<Q>`:
[01:06:07]    |
[01:06:07]    |
[01:06:07] LL | impl AsRef<Q> for Box<Q> { //~ ERROR conflicting implementations
[01:06:07]    |
[01:06:07]    = note: conflicting implementation in crate `alloc`:
[01:06:07]    = note: conflicting implementation in crate `alloc`:
[01:06:07]            - impl<T> std::convert::AsRef<T> for std::boxed::Box<T>
[01:06:07]              where T: ?Sized;
[01:06:07] 
[01:06:07] error[E0119]: conflicting implementations of trait `std::convert::From<S>` for type `S`:
[01:06:07]    |
[01:06:07]    |
[01:06:07] LL | impl From<S> for S { //~ ERROR conflicting implementations
[01:06:07]    |
[01:06:07]    = note: conflicting implementation in crate `core`:
[01:06:07]    = note: conflicting implementation in crate `core`:
[01:06:07]            - impl<T> std::convert::From<T> for T;
[01:06:07] 
[01:06:07] error[E0119]: conflicting implementations of trait `std::convert::TryFrom<X>` for type `X`:
[01:06:07]    |
[01:06:07]    |
[01:06:07] LL | impl TryFrom<X> for X { //~ ERROR conflicting implementations
[01:06:07]    |
[01:06:07]    = note: conflicting implementation in crate `core`:
[01:06:07]    = note: conflicting implementation in crate `core`:
[01:06:07]            - impl<T, U> std::convert::TryFrom<U> for T
[01:06:07]              where U: std::convert::Into<T>;
[01:06:07] error: aborting due to 3 previous errors
[01:06:07] 
[01:06:07] For more information about this error, try `rustc --explain E0119`.
[01:06:07] 
---
[01:06:07] 1 error[E0109]: type arguments are not allowed for this type
[01:06:07] -   --> $DIR/mod-subitem-as-enum-variant.rs:8:11
[01:06:07] +   --> $DIR/mod-subitem-as-enum-variant.rs:7:11
[01:06:07] 3    |
[01:06:07] 4 LL |     Mod::<i32>::FakeVariant(0);
[01:06:07] 
[01:06:07] 
[01:06:07] The actual stderr differed from the expected stderr.
[01:06:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod-subitem-as-enum-variant/mod-subitem-as-enum-variant.stderr
[01:06:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod-subitem-as-enum-variant/mod-subitem-as-enum-variant.stderr
[01:06:07] To update references, rerun the tests and pass the `--bless` flag
[01:06:07] To only update this specific test, also pass `--test-args mod-subitem-as-enum-variant.rs`
[01:06:07] error: 1 errors occurred comparing output.
[01:06:07] status: exit code: 1
[01:06:07] status: exit code: 1
[01:06:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mod-subitem-as-enum-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod-subitem-as-enum-variant/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod-subitem-as-enum-variant/auxiliary" "-A" "unused"
[01:06:07] ------------------------------------------
[01:06:07] 
[01:06:07] ------------------------------------------
[01:06:07] stderr:
[01:06:07] stderr:
[01:06:07] ------------------------------------------
[01:06:07] error[E0109]: type arguments are not allowed for this type
[01:06:07]   --> /checkout/src/test/ui/mod-subitem-as-enum-variant.rs:7:11
[01:06:07]    |
[01:06:07] LL |     Mod::<i32>::FakeVariant(0);
[01:06:07] 
[01:06:07] error: aborting due to previous error
[01:06:07] 
[01:06:07] For more information about this error, try `rustc --explain E0109`.
---
[01:06:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:06:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:07] 
[01:06:07] 
[01:06:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:07] 
[01:06:07] 
[01:06:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:07] Build completed unsuccessfully in 0:04:20
[01:06:07] Build completed unsuccessfully in 0:04:20
[01:06:07] Makefile:48: recipe for target 'check' failed
[01:06:07] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0856bed3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 22 15:59:39 UTC 2019
---
travis_time:end:058a579b:start=1555948780789011249,finish=1555948780793857299,duration=4846050
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04c7cd7b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01156795
travis_time:start:01156795
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1276b7e0
$ dmesg | grep -i kill
