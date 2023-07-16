plain
travis_time:end:10b45dc0:start=1559216656207704247,finish=1559216743868434663,duration=87660730416
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:08:26] .................................................................................................... 1200/5599
[01:08:29] .................................................................................................... 1300/5599
[01:08:31] .................................................................................................... 1400/5599
[01:08:34] .................................................................................................... 1500/5599
[01:08:37] ...................................F................................................................ 1600/5599
[01:08:44] .................................................................................................... 1800/5599
[01:08:47] .................................................................................................... 1900/5599
[01:08:51] .................................................................................................... 2000/5599
[01:08:54] ..................................................................................i................. 2100/5599
---
[01:11:13] ---- [ui] ui/feature-gates/feature-gate-doc_cfg.rs stdout ----
[01:11:13] 
[01:11:13] error: ui test compiled successfully!
[01:11:13] status: exit code: 0
[01:11:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-doc_cfg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-doc_cfg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-doc_cfg/auxiliary" "-A" "unused"
[01:11:13] ------------------------------------------
[01:11:13] 
[01:11:13] ------------------------------------------
[01:11:13] stderr:
---
[01:11:13] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:11:13] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:13] 
[01:11:13] 
[01:11:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:13] 
[01:11:13] 
[01:11:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:13] Build completed unsuccessfully in 0:04:46
[01:11:13] Build completed unsuccessfully in 0:04:46
[01:11:13] make: *** [check] Error 1
[01:11:13] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0817ba86
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 12:57:06 UTC 2019
