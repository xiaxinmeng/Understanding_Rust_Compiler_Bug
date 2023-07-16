plain
travis_time:end:10dd4c84:start=1560294739297847544,finish=1560294740197612609,duration=899765065
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:59:29] .................................................................................................... 4900/5669
[00:59:33] .................................................................................................... 5000/5669
[00:59:36] .................................................................................................... 5100/5669
[00:59:40] .................................................................................................... 5200/5669
[00:59:44] ................F................................................................................... 5300/5669
[00:59:50] .................................................................................................... 5500/5669
[00:59:53] .................................................................................................... 5600/5669
[00:59:55] .......i.............................................................
[00:59:55] failures:
[00:59:55] failures:
[00:59:55] 
[00:59:55] ---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
[00:59:55] 
[00:59:55] error: ui test compiled successfully!
[00:59:55] status: exit code: 0
[00:59:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
[00:59:55] ------------------------------------------
[00:59:55] 
[00:59:55] ------------------------------------------
[00:59:55] stderr:
---
[00:59:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:55] 
[00:59:55] 
[00:59:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:55] 
[00:59:55] 
[00:59:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:55] Build completed unsuccessfully in 0:55:03
