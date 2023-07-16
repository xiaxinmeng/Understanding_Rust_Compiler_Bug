plain
travis_time:end:0a1d2e4a:start=1559297294266978279,finish=1559297383471241724,duration=89204263445
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:17] .................................................................................................... 300/5607
[00:55:20] .................................................................................................... 400/5607
[00:55:24] ...................................................................................................i 500/5607
[00:55:27] .................................................................................................... 600/5607
[00:55:32] .......................................................................F............................ 700/5607
[00:55:41] .........................................................................................i.......... 900/5607
[00:55:45] .i.................................................................................................. 1000/5607
[00:55:49] ..................iiiii............................................................................. 1100/5607
[00:55:51] .................................................................................................... 1200/5607
---
[00:58:38] 5    |            ^^^^^^^^^^^^^^
[00:58:38] 
[00:58:38] 
[00:58:38] The actual stderr differed from the expected stderr.
[00:58:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/broken-mir-2/broken-mir-2.stderr
[00:58:38] To update references, rerun the tests and pass the `--bless` flag
[00:58:38] To only update this specific test, also pass `--test-args const-generics/broken-mir-2.rs`
[00:58:38] error: 1 errors occurred comparing output.
[00:58:38] status: exit code: 0
[00:58:38] status: exit code: 0
[00:58:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/broken-mir-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/broken-mir-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/broken-mir-2/auxiliary" "-A" "unused"
[00:58:38] ------------------------------------------
[00:58:38] 
[00:58:38] ------------------------------------------
[00:58:38] stderr:
---
[00:58:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:38] 
[00:58:38] 
[00:58:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:38] 
[00:58:38] 
[00:58:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:38] Build completed unsuccessfully in 0:54:15
---
travis_time:end:005d9579:start=1559300912043375152,finish=1559300912050377969,duration=7002817
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1508f4a2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06b3ac6e
$ dmesg | grep -i kill
