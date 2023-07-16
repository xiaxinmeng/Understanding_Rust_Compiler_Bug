plain
travis_time:end:038864ac:start=1560704341607239672,finish=1560704431399135379,duration=89791895707
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:57:17] .................................................................................................... 1200/5682
[00:57:19] .................................................................................................... 1300/5682
[00:57:22] .................................................................................................... 1400/5682
[00:57:25] .................................................................................................... 1500/5682
[00:57:28] ..........................F......................................................................... 1600/5682
[00:57:34] ...i................................................................................................ 1800/5682
[00:57:38] .................................................................................................... 1900/5682
[00:57:41] .................................................................................................... 2000/5682
[00:57:45] .................................................................................................... 2100/5682
---
[01:00:11] error: /checkout/src/test/ui/fat-ptr-cast.rs:26: expected error not found: E0277
[01:00:11] 
[01:00:11] error: 0 unexpected errors found, 1 expected errors not found
[01:00:11] status: exit code: 1
[01:00:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fat-ptr-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-ptr-cast/auxiliary" "-A" "unused"
[01:00:11]     Error {
[01:00:11]         line_num: 26,
[01:00:11]         kind: Some(
[01:00:11]             Error,
---
[01:00:11] 
[01:00:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:11] 
[01:00:11] 
[01:00:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:11] 
[01:00:11] 
[01:00:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:11] Build completed unsuccessfully in 0:56:12
---
travis_time:end:00c85192:start=1560708053565137874,finish=1560708053572057214,duration=6919340
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20bac2b3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b51f58
$ dmesg | grep -i kill
