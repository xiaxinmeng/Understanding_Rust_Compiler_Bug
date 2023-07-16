plain
travis_time:end:265ea4c8:start=1556904513689761101,finish=1556904598561650806,duration=84871889705
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:20] 
[01:18:20] running 42 tests
[01:18:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:18:39] ...................................F......
[01:18:39] 
[01:18:39] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:18:39] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:18:39] thread '[mir-opt] mir-opt/storage_ranges.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/storage_ranges/rustc.main.TypeckMir.before.mir` from test does not exist', src/tools/compiletest/src/runtest.rs:3024:13
[01:18:39] 
[01:18:39] 
[01:18:39] failures:
[01:18:39]     [mir-opt] mir-opt/storage_ranges.rs
[01:18:39]     [mir-opt] mir-opt/storage_ranges.rs
[01:18:39] 
[01:18:39] test result: FAILED. 41 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:39] 
[01:18:39] 
[01:18:39] 
[01:18:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:39] 
[01:18:39] 
[01:18:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:39] Build completed unsuccessfully in 0:11:20
[01:18:39] Build completed unsuccessfully in 0:11:20
[01:18:39] Makefile:48: recipe for target 'check' failed
[01:18:39] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00010860
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May  3 18:48:47 UTC 2019
Fri May  3 18:48:47 UTC 2019
5473
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04834227
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:015c6dc6
travis_time:start:015c6dc6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:119522ec
$ dmesg | grep -i kill
