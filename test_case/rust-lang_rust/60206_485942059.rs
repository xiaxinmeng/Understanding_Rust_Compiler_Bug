plain
travis_time:end:1fb42390:start=1556043587341374336,finish=1556043713085447566,duration=125744073230
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:40] ..........................................................................................i......... 500/5462
[01:03:44] .................................................................................................... 600/5462
[01:03:47] .................................................................................................... 700/5462
[01:03:52] .................................................................................................... 800/5462
[01:03:58] ...........F............................................i...............i........................... 900/5462
[01:04:01] .........................................................................................iiiii...... 1000/5462
[01:04:08] .................................................................................................... 1200/5462
[01:04:10] .................................................................................................... 1300/5462
[01:04:13] .................................................................................................... 1400/5462
[01:04:16] .................................................................................................... 1500/5462
---
[01:06:47] - error[E0723]: function pointers in const fn are unstable (see issue #57563)
[01:06:47] + error[E0723]: function pointers in const fn are unstable
[01:06:47] 2   --> $DIR/allow_const_fn_ptr.rs:4:16
[01:06:47] 3    |
[01:06:47] 4 LL | const fn error(_: fn()) {}
[01:06:47] 5    |                ^
[01:06:47] 6    |
[01:06:47] +    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:06:47] 7    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:06:47] 7    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:06:47] 8 
[01:06:47] 9 error: aborting due to previous error
[01:06:47] 
[01:06:47] 
[01:06:47] The actual stderr differed from the expected stderr.
[01:06:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
[01:06:47] To update references, rerun the tests and pass the `--bless` flag
[01:06:47] To only update this specific test, also pass `--test-args consts/min_const_fn/allow_const_fn_ptr.rs`
[01:06:47] error: 1 errors occurred comparing output.
[01:06:47] status: exit code: 1
[01:06:47] status: exit code: 1
[01:06:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/auxiliary" "-A" "unused"
[01:06:47] ------------------------------------------
[01:06:47] 
[01:06:47] ------------------------------------------
[01:06:47] stderr:
[01:06:47] stderr:
[01:06:47] ------------------------------------------
[01:06:47] error[E0723]: function pointers in const fn are unstable
[01:06:47]   --> /checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs:4:16
[01:06:47]    |
[01:06:47] LL | const fn error(_: fn()) {} //~ ERROR function pointers in const fn are unstable
[01:06:47]    |
[01:06:47]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:06:47]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:06:47] 
---
[01:06:47] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:06:47] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:47] 
[01:06:47] 
[01:06:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:47] 
[01:06:47] 
[01:06:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:47] Build completed unsuccessfully in 0:04:24
[01:06:47] Build completed unsuccessfully in 0:04:24
[01:06:47] Makefile:48: recipe for target 'check' failed
[01:06:47] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:271986a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 19:28:49 UTC 2019
