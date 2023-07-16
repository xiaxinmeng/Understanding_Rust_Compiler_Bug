plain
travis_time:end:0a83cb57:start=1557583605861592557,finish=1557583693864257711,duration=88002665154
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:25] .................................................................................................... 3600/5524
[01:14:29] .................................................................................................... 3700/5524
[01:14:32] ......................................................................................ii............ 3800/5524
[01:14:35] .................................................................................................... 3900/5524
[01:14:38] ........i............................................................F.............................. 4000/5524
[01:14:40] .......................................................................i............................ 4100/5524
[01:14:42] ........................................................F........................................... 4200/5524
[01:14:57] .................................................................................................... 4400/5524
[01:15:01] .................................................................................................... 4500/5524
[01:15:04] .................................................................................................... 4600/5524
[01:15:09] .................................................................................................... 4700/5524
---
[01:15:40] ---- [ui] ui/parser/issue-17383.rs stdout ----
[01:15:40] 
[01:15:40] error: ui test compiled successfully!
[01:15:40] status: exit code: 0
[01:15:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-17383.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-17383" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-17383/auxiliary" "-A" "unused"
[01:15:40] ------------------------------------------
[01:15:40] 
[01:15:40] ------------------------------------------
[01:15:40] stderr:
---
[01:15:40] ---- [ui] ui/parser/tag-variant-disr-non-nullary.rs stdout ----
[01:15:40] 
[01:15:40] error: ui test compiled successfully!
[01:15:40] status: exit code: 0
[01:15:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/tag-variant-disr-non-nullary.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/tag-variant-disr-non-nullary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/tag-variant-disr-non-nullary/auxiliary" "-A" "unused"
[01:15:40] ------------------------------------------
[01:15:40] 
[01:15:40] ------------------------------------------
[01:15:40] stderr:
---
[01:15:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:15:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:40] 
[01:15:40] 
[01:15:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:40] 
[01:15:40] 
[01:15:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:40] Build completed unsuccessfully in 0:04:48
[01:15:40] Build completed unsuccessfully in 0:04:48
[01:15:40] Makefile:48: recipe for target 'check' failed
[01:15:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01414c8c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 11 15:24:04 UTC 2019
---
travis_time:end:04c79209:start=1557588245415754153,finish=1557588245422937059,duration=7182906
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cbda388
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02551a73
$ dmesg | grep -i kill
