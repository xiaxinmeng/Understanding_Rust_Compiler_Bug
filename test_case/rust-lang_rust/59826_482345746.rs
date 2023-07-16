plain
travis_time:end:2d3a7558:start=1555016424559313229,finish=1555016518699148313,duration=94139835084
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:08:31] .................................................................................................... 4400/5530
[01:08:35] .................................................................................................... 4500/5530
[01:08:39] .................................................................................................... 4600/5530
[01:08:43] .................................................................................................... 4700/5530
[01:08:50] ................................F................................................................... 4800/5530
[01:08:57] .................................................................................................... 5000/5530
[01:09:02] .................................................................................................... 5100/5530
[01:09:06] .................................................................................................... 5200/5530
[01:09:10] .................................................................................................... 5300/5530
---
[01:09:18] ------------------------------------------
[01:09:18] stderr:
[01:09:18] ------------------------------------------
[01:09:18] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:09:18]   left: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":29] b = Point {", "    x: 42,", "    y: 24,", "}", ":37]", ":41] &a = NoCopy(", "    1337,", ")", ":41] dbg!(& a) = NoCopy(", "    1337,", ")", ":46] f(&42) = 42", "before", ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":59] \"Yeah\" = \"Yeah\"", ":62] 1 = 1", ":62] 2 = 2", ":66] 1u8 = 1", ":66] 2u32 = 2", ":66] \"Yeah\" = \"Yeah\""]`,
[01:09:18]  right: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":29] b = Point {", "    x: 42,", "    y: 24,", "}", ":37]", ":41] &a = NoCopy(", "    1337,", ")", ":41] dbg!(& a) = NoCopy(", "    1337,", ")", ":46] f(&42) = 42", "before", ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":59] dbg!(\"Yeah\",) = \"Yeah\"", ":62] 1 = 1", ":63] 2 = 2", ":66] 1u8 = 1", ":66] 2u32 = 2", ":66] \"Yeah\" = \"Yeah\""]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:71:5
[01:09:18] 
[01:09:18] ------------------------------------------
[01:09:18] 
[01:09:18] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:09:18] 
[01:09:18] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:09:18] 
[01:09:18] 
[01:09:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:18] 
[01:09:18] 
[01:09:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:18] Build completed unsuccessfully in 0:04:49
[01:09:18] Build completed unsuccessfully in 0:04:49
[01:09:18] Makefile:48: recipe for target 'check' failed
[01:09:18] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07c86ad4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 22:11:26 UTC 2019
---
travis_time:end:0b5d9b5f:start=1555020688991145608,finish=1555020688999124936,duration=7979328
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01766eb0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003d9450
$ dmesg | grep -i kill
