plain
travis_time:end:058fe486:start=1560528976814817781,finish=1560528979090157331,duration=2275339550
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:57:36] .................................................................................................... 1200/5679
[00:57:38] .................................................................................................... 1300/5679
[00:57:41] .................................................................................................... 1400/5679
[00:57:44] .................................................................................................... 1500/5679
[00:57:47] ................................................................................F................... 1600/5679
[00:57:53] ..i................................................................................................. 1800/5679
[00:57:57] .................................................................................................... 1900/5679
[00:58:01] .................................................................................................... 2000/5679
[00:58:04] .................................................................................................... 2100/5679
---
[01:00:30] ---- [ui] ui/feature-gates/feature-gate-const_transmute.rs stdout ----
[01:00:30] 
[01:00:30] error: ui test compiled successfully!
[01:00:30] status: exit code: 0
[01:00:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_transmute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_transmute" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_transmute/auxiliary" "-A" "unused"
[01:00:30] ------------------------------------------
[01:00:30] 
[01:00:30] ------------------------------------------
[01:00:30] stderr:
---
[01:00:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:30] 
[01:00:30] 
[01:00:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:30] 
[01:00:30] 
[01:00:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:30] Build completed unsuccessfully in 0:55:49
---
travis_time:end:05ee9758:start=1560532621606383209,finish=1560532621611734553,duration=5351344
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2d5305e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004f47d8
travis_time:start:004f47d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:151707f0
$ dmesg | grep -i kill
