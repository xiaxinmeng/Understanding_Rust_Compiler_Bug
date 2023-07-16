plain
travis_time:end:01efb1fa:start=1556044724789612055,finish=1556044823895987550,duration=99106375495
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:04:08] ..........................................................................................i......... 500/5462
[01:04:12] .................................................................................................... 600/5462
[01:04:15] .................................................................................................... 700/5462
[01:04:21] .................................................................................................... 800/5462
[01:04:26] ..........F.............................................i...............i........................... 900/5462
[01:04:29] .........................................................................................iiiii...... 1000/5462
[01:04:36] .................................................................................................... 1200/5462
[01:04:39] .................................................................................................... 1300/5462
[01:04:42] .................................................................................................... 1400/5462
[01:04:45] .................................................................................................... 1500/5462
---
[01:07:20] - error[E0723]: function pointers in const fn are unstable (see issue #57563)
[01:07:20] + error[E0723]: function pointers in const fn are unstable
[01:07:20] 2   --> $DIR/allow_const_fn_ptr.rs:4:16
[01:07:20] 3    |
[01:07:20] 4 LL | const fn error(_: fn()) {}
[01:07:20] 5    |                ^
[01:07:20] 6    |
[01:07:20] +    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:07:20] 7    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:07:20] 7    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:07:20] 8 
[01:07:20] 9 error: aborting due to previous error
[01:07:20] 
[01:07:20] 
[01:07:20] The actual stderr differed from the expected stderr.
[01:07:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
[01:07:20] To update references, rerun the tests and pass the `--bless` flag
[01:07:20] To only update this specific test, also pass `--test-args consts/min_const_fn/allow_const_fn_ptr.rs`
[01:07:20] error: 1 errors occurred comparing output.
[01:07:20] status: exit code: 1
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/auxiliary" "-A" "unused"
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] error[E0723]: function pointers in const fn are unstable
[01:07:20]   --> /checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs:4:16
[01:07:20]    |
[01:07:20] LL | const fn error(_: fn()) {} //~ ERROR function pointers in const fn are unstable
[01:07:20]    |
[01:07:20]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:07:20]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:07:20] 
---
[01:07:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:07:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:20] 
[01:07:20] 
[01:07:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:20] 
[01:07:20] 
[01:07:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:20] Build completed unsuccessfully in 0:04:30
[01:07:20] Build completed unsuccessfully in 0:04:30
[01:07:20] Makefile:48: recipe for target 'check' failed
[01:07:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ae34086
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 19:47:53 UTC 2019
---
travis_time:end:0177c040:start=1556048875080163872,finish=1556048875085049343,duration=4885471
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04c98741
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f3f2970
travis_time:start:1f3f2970
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05106c18
$ dmesg | grep -i kill
