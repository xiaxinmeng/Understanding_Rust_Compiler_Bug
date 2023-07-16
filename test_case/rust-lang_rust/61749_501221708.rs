plain
travis_time:end:0f63872d:start=1560333044749588691,finish=1560333135047260837,duration=90297672146
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:56:49] .................................................................................................... 400/5678
[00:56:52] .................................................................................................... 500/5678
[00:56:56] ...................................i................................................................ 600/5678
[00:57:00] .................................................................................................... 700/5678
[00:57:04] ..................................F................................................................. 800/5678
[00:57:14] ...........................................i...........i............................................ 1000/5678
[00:57:17] ........................................................................iiiii....................... 1100/5678
[00:57:21] .................................................................................................... 1200/5678
[00:57:23] .................................................................................................... 1300/5678
---
[01:00:12] 5    |            ^^^^^^^^^^^^^^
[01:00:12] 6 
[01:00:12] - error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
[01:00:12] -   --> $DIR/issue-61336.rs:9:5
[01:00:12] + error: array lengths can't depend on generic parameters
[01:00:12] 9    |
[01:00:12] 9    |
[01:00:12] 10 LL |     [x; N]
[01:00:12] -    |     ^^^^^^ the trait `std::marker::Copy` is not implemented for `T`
[01:00:12] + 
[01:00:12] + 
[01:00:12] + error: array lengths can't depend on generic parameters
[01:00:12] 12    |
[01:00:12] -    = help: consider adding a `where T: std::marker::Copy` bound
[01:00:12] -    = note: the `Copy` trait is required because the repeated element will be copied
[01:00:12] -    = note: the `Copy` trait is required because the repeated element will be copied
[01:00:12] + LL |     [x; N]
[01:00:12] 15 
[01:00:12] - error: aborting due to previous error
[01:00:12] + error: aborting due to 2 previous errors
[01:00:12] 17 
[01:00:12] 17 
[01:00:12] - For more information about this error, try `rustc --explain E0277`.
[01:00:12] 19 
[01:00:12] 
[01:00:12] 
[01:00:12] The actual stderr differed from the expected stderr.
[01:00:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-61336/issue-61336.stderr
[01:00:12] To update references, rerun the tests and pass the `--bless` flag
[01:00:12] To only update this specific test, also pass `--test-args const-generics/issue-61336.rs`
[01:00:12] error: 1 errors occurred comparing output.
[01:00:12] status: exit code: 1
[01:00:12] status: exit code: 1
[01:00:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-61336.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-61336" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-61336/auxiliary" "-A" "unused"
[01:00:12] ------------------------------------------
[01:00:12] 
[01:00:12] ------------------------------------------
[01:00:12] stderr:
[01:00:12] stderr:
[01:00:12] ------------------------------------------
[01:00:12] warning: the feature `const_generics` is incomplete and may cause the compiler to crash
[01:00:12]   --> /checkout/src/test/ui/const-generics/issue-61336.rs:1:12
[01:00:12]    |
[01:00:12] LL | #![feature(const_generics)]
[01:00:12]    |            ^^^^^^^^^^^^^^
[01:00:12] 
[01:00:12] error: array lengths can't depend on generic parameters
[01:00:12]    |
[01:00:12]    |
[01:00:12] LL |     [x; N]
[01:00:12] 
[01:00:12] 
[01:00:12] error: array lengths can't depend on generic parameters
[01:00:12]    |
[01:00:12]    |
[01:00:12] LL |     [x; N]
[01:00:12] 
[01:00:12] error: aborting due to 2 previous errors
[01:00:12] 
[01:00:12] 
---
[01:00:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:12] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:12] 
[01:00:12] 
[01:00:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:12] 
[01:00:12] 
[01:00:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:12] Build completed unsuccessfully in 0:56:00
---
travis_time:end:11ea5cca:start=1560336758377335251,finish=1560336758382347724,duration=5012473
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02240859
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:cr
