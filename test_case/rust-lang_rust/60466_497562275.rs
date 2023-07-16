plain
travis_time:end:1fea44d8:start=1559269824347602299,finish=1559269912689337655,duration=88341735356
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:23] .................................................................................................... 300/5607
[00:54:26] .................................................................................................... 400/5607
[00:54:29] ...................................................................................................i 500/5607
[00:54:33] .................................................................................................... 600/5607
[00:54:37] .......................................................................F............................ 700/5607
[00:54:46] .........................................................................................i.......... 900/5607
[00:54:50] .i.................................................................................................. 1000/5607
[00:54:53] ..................iiiii............................................................................. 1100/5607
[00:54:56] .................................................................................................... 1200/5607
---
[00:57:39] ---- [ui] ui/const-generics/broken-mir-2.rs stdout ----
[00:57:39] 
[00:57:39] error: ui test compiled successfully!
[00:57:39] status: exit code: 0
[00:57:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/broken-mir-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/broken-mir-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/broken-mir-2/auxiliary" "-A" "unused"
[00:57:39] ------------------------------------------
[00:57:39] 
[00:57:39] ------------------------------------------
[00:57:39] stderr:
---
[00:57:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:57:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:57:39] 
[00:57:39] 
[00:57:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:39] 
[00:57:39] 
[00:57:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:39] Build completed unsuccessfully in 0:53:19
---
travis_time:end:2c6fbe4a:start=1559273382426577729,finish=1559273382431312479,duration=4734750
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b5aeed8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:cra
