plain
travis_time:end:006c93cc:start=1555943989932350954,finish=1555943991974953965,duration=2042603011
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:25] .................................................................................................... 900/2961
[01:09:41] .................................................................................................... 1000/2961
[01:09:54] .................................................................................................... 1100/2961
[01:10:04] .................................................................................................... 1200/2961
[01:10:14] ............F....................................................................................... 1300/2961
[01:10:39] .................................................................................................... 1500/2961
[01:10:48] .................................................................................i.................. 1600/2961
[01:11:01] .................................................................................................... 1700/2961
[01:11:15] .................................................................................................... 1800/2961
---
[01:14:19] ------------------------------------------
[01:14:19] stderr:
[01:14:19] ------------------------------------------
[01:14:19] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:14:19]   left: `("[u8]", "str", "dyn core::markerSend", "issue_21058::NT", "issue_21058::DST")`,
[01:14:19]  right: `("[u8]", "str", "dyn core::marker::Send", "issue_21058::NT", "issue_21058::DST")`', /checkout/src/test/run-pass/issues/issue-21058.rs:10:5
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] 
[01:14:19] 
---
[01:14:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:14:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:19] 
[01:14:19] 
[01:14:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:19] 
[01:14:19] 
[01:14:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:19] Build completed unsuccessfully in 0:11:01
[01:14:19] Build completed unsuccessfully in 0:11:01
[01:14:19] make: *** [check] Error 1
[01:14:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08225a60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 22 15:54:22 UTC 2019
---
travis_time:end:1b0d045c:start=1555948464223013275,finish=1555948464227402371,duration=4389096
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008c29d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00839e79
travis_time:start:00839e79
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08e6c818
$ dmesg | grep -i kill
