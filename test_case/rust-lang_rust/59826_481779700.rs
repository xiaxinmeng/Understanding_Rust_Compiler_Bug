plain
travis_time:end:0090840b:start=1554912061081256661,finish=1554912064486806059,duration=3405549398
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:54] .................................................................................................... 4400/5530
[01:09:57] .................................................................................................... 4500/5530
[01:10:01] .................................................................................................... 4600/5530
[01:10:05] .................................................................................................... 4700/5530
[01:10:11] ..................................F................................................................. 4800/5530
[01:10:18] .................................................................................................... 5000/5530
[01:10:23] .................................................................................................... 5100/5530
[01:10:26] .................................................................................................... 5200/5530
[01:10:30] .................................................................................................... 5300/5530
---
[01:10:37] ------------------------------------------
[01:10:37] stderr:
[01:10:37] ------------------------------------------
[01:10:37] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:10:37]   left: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":29] b = Point {", "    x: 42,", "    y: 24,", "}", ":37]", ":41] &a = NoCopy(", "    1337,", ")", ":41] dbg!(& a) = NoCopy(", "    1337,", ")", ":46] f(&42) = 42", "before", ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":59] \"Yeah\" = \"Yeah\"", ":62] 1 = 1", ":62] 2 = 2", ":66] 1u8 = 1", ":66] 2u32 = 2", ":66] \"Yeah\" = \"Yeah\""]`,
[01:10:37]  right: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":29] b = Point {", "    x: 42,", "    y: 24,", "}", ":37]", ":41] &a = NoCopy(", "    1337,", ")", ":41] dbg!(& a) = NoCopy(", "    1337,", ")", ":46] f(&42) = 42", "before", ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":59] dbg!(\"Yeah\",) = \"Yeah\"", ":62] dbg!(1) = 1", ":63] dbg!(2) = 2", ":66] dbg!(1) = 1", ":66] dbg!(2) = 2", ":67] dbg!(\"Yeah\",) = \"Yeah\""]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:71:5
[01:10:37] 
[01:10:37] ------------------------------------------
[01:10:37] 
[01:10:37] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:10:37] 
[01:10:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:10:37] 
[01:10:37] 
[01:10:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:37] 
[01:10:37] 
[01:10:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:37] Build completed unsuccessfully in 0:04:30
[01:10:37] Build completed unsuccessfully in 0:04:30
[01:10:37] make: *** [check] Error 1
[01:10:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:265cb39c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 17:11:51 UTC 2019
---
travis_time:end:11e888aa:start=1554916313018693427,finish=1554916313023562649,duration=4869222
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005db29a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:022f1bac
travis_time:start:022f1bac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:179d97db
$ dmesg | grep -i kill
