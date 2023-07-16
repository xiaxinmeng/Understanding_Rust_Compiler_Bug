plain
travis_time:end:08509b66:start=1556795034790578405,finish=1556795035558249112,duration=767670707
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:56] .................................................................................................... 300/5483
[01:12:00] .................................................................................................... 400/5483
[01:12:03] .............................................................................................i...... 500/5483
[01:12:07] .................................................................................................... 600/5483
[01:12:10] ......................................................................................F............. 700/5483
[01:12:21] .................................................................i...............i.................. 900/5483
[01:12:24] ..................................................................................................ii 1000/5483
[01:12:29] iii................................................................................................. 1100/5483
[01:12:31] .................................................................................................... 1200/5483
---
[01:15:14] error: /checkout/src/test/ui/consts/const-err.rs:14: unexpected error: '14:21: 14:24: erroneous constant used [E0080]'
[01:15:14] 
[01:15:14] error: 1 unexpected errors found, 0 expected errors not found
[01:15:14] status: exit code: 1
[01:15:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zforce-overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err/auxiliary" "-A" "unused"
[01:15:14]     Error {
[01:15:14]         line_num: 14,
[01:15:14]         kind: Some(
[01:15:14]             Error,
---
[01:15:14] 
[01:15:14] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:15:14] 
[01:15:14] 
[01:15:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:14] 
[01:15:14] 
[01:15:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:14] Build completed unsuccessfully in 0:04:30
[01:15:14] Build completed unsuccessfully in 0:04:30
[01:15:14] Makefile:48: recipe for target 'check' failed
[01:15:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:009d8c06
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 12:19:22 UTC 2019
---
travis_time:end:12b714c2:start=1556799563533067651,finish=1556799563537941921,duration=4874270
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b30aa51
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:211c8396
travis_time:start:211c8396
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f104861
$ dmesg | grep -i kill
