plain
travis_time:end:26202e63:start=1560471102759315273,finish=1560471103555535998,duration=796220725
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:01] 
[00:57:01] running 2921 tests
[00:57:12] .................................................................................................... 100/2921
[00:57:15] ERROR 2019-06-14T01:09:09Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/run-pass/async-await/auxiliary/arc_wake.rs` source not found"
[00:57:23] ...............................................F............................i....................... 200/2921
[00:57:42] .................................................................................................... 400/2921
[00:57:51] .................................................................................................... 500/2921
[00:58:02] .................................................................................................... 600/2921
[00:58:16] .................................................................................................... 700/2921
---
[01:03:17] failures:
[01:03:17] 
[01:03:17] ---- [run-pass] run-pass/async-await/async-fn-size.rs stdout ----
[01:03:17] 
[01:03:17] error: aux-build `/checkout/src/test/run-pass/async-await/auxiliary/arc_wake.rs` source not found
[01:03:17] thread '[run-pass] run-pass/async-await/async-fn-size.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2214:9
[01:03:17] 
[01:03:17] 
[01:03:17] failures:
[01:03:17]     [run-pass] run-pass/async-await/async-fn-size.rs
[01:03:17]     [run-pass] run-pass/async-await/async-fn-size.rs
[01:03:17] 
[01:03:17] test result: FAILED. 2911 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
[01:03:17] 
[01:03:17] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:03:17] 
[01:03:17] 
[01:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:17] 
[01:03:17] 
[01:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:17] Build completed unsuccessfully in 0:58:24
---
travis_time:end:02187824:start=1560474913723457227,finish=1560474913728711908,duration=5254681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:198297e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a26b500
travis_time:start:0a26b500
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2998fa5e
$ dmesg | grep -i kill
