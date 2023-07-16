plain
travis_time:end:32136ab9:start=1557190013426422078,finish=1557190014194054855,duration=767632777
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:58] .................................................................................................... 4000/5496
[01:13:00] ......................................................i............................................. 4100/5496
[01:13:03] .................................................................................................... 4200/5496
[01:13:11] .................................................................................................... 4300/5496
[01:13:18] ........F........................................................................................... 4400/5496
[01:13:25] .................................................................................................... 4600/5496
[01:13:30] .................................................................................................... 4700/5496
[01:13:36] .................................................................................................... 4800/5496
[01:13:39] .................................................................................................... 4900/5496
---
[01:14:00] ---- [ui] ui/proc-macro/no-missing-docs.rs stdout ----
[01:14:00] 
[01:14:00] error: ui test compiled successfully!
[01:14:00] status: exit code: 0
[01:14:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/no-missing-docs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/auxiliary" "-A" "unused"
[01:14:00] ------------------------------------------
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] stderr:
---
[01:14:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:14:00] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:00] 
[01:14:00] 
[01:14:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:00] 
[01:14:00] 
[01:14:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:00] Build completed unsuccessfully in 0:04:31
[01:14:00] Build completed unsuccessfully in 0:04:31
[01:14:00] make: *** [check] Error 1
[01:14:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:157fafb4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May  7 02:01:06 UTC 2019
---
travis_time:end:004554aa:start=1557194467539389480,finish=1557194467548611035,duration=9221555
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03f8e2fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:088f680f
travis_time:start:088f680f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1394c9ca
$ dmesg | grep -i kill
