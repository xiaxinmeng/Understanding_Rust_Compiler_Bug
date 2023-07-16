plain
travis_time:end:05332188:start=1559333303814721910,finish=1559333306301868588,duration=2487146678
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:00] 
[00:57:00] running 5609 tests
[00:57:03] .........F.......................................................................................... 100/5609
[00:57:13] .................................................................................................... 300/5609
[00:57:17] .................................................................................................... 400/5609
[00:57:20] .................................................................................................... 500/5609
[00:57:24] i................................................................................................... 600/5609
---
[01:00:42]   |
[01:00:42] 
[01:00:42] 
[01:00:42] The actual stdout differed from the expected stdout.
[01:00:42] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type/missing-type.stdout
[01:00:42] To update references, rerun the tests and pass the `--bless` flag
[01:00:42] To only update this specific test, also pass `--test-args annotate-snippet/missing-type.rs`
[01:00:42] error: 1 errors occurred comparing output.
[01:00:42] status: exit code: 1
[01:00:42] status: exit code: 1
[01:00:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/annotate-snippet/missing-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/annotate-snippet/missing-type/auxiliary" "-A" "unused"
[01:00:42] ------------------------------------------
[01:00:42] error[E0412]: cannot find type `Iter` in this scope
[01:00:42]  --> /checkout/src/test/ui/annotate-snippet/missing-type.rs:4:11
[01:00:42]   |
---
[01:00:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:42] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:42] 
[01:00:42] 
[01:00:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:42] 
[01:00:42] 
[01:00:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:42] Build completed unsuccessfully in 0:56:01
---
travis_time:end:150e1d06:start=1559336961463716439,finish=1559336961469035418,duration=5318979
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05ef3cac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
