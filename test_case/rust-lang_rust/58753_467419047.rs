plain
travis_time:end:0a024548:start=1551180202029543386,finish=1551180203226046097,duration=1196502711
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:27] .................................................................................................... 4300/5417
[01:02:30] .................................................................................................... 4400/5417
[01:02:33] .................................................................................................... 4500/5417
[01:02:37] ...............................................................i.................................... 4600/5417
[01:02:43] ....................................F............................................................... 4700/5417
[01:02:49] .................................................................................................... 4900/5417
[01:02:53] .................................................................................................... 5000/5417
[01:02:56] .................................................................................................... 5100/5417
[01:02:59] .................................................................................................... 5200/5417
---
[01:03:05] ------------------------------------------
[01:03:05] stderr:
[01:03:05] ------------------------------------------
[01:03:05] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:03:05]   left: `[":0021] Unit = Unit", ":0022] a = Unit", ":0028] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24", "}", ":0029] b = Point {", "    x: 42,", "    y: 24", "}", ":0038] &a = NoCopy(", "    1337", ")", ":0038] dbg!(& a) = NoCopy(", "    1337", ")", ":0043] f(&42) = 42", "before", ":0048] { foo += 1; eprintln!(\"before\"); 7331 } = 7331"]`,
[01:03:05]  right: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24", "}", ":29] b = Point {", "    x: 42,", "    y: 24", "}", ":38] &a = NoCopy(", "    1337", ")", ":38] dbg!(& a) = NoCopy(", "    1337", ")", ":43] f(&42) = 42", "before", ":48] { foo += 1; eprintln!(\"before\"); 7331 } = 7331"]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:57:5
[01:03:05] 
[01:03:05] ------------------------------------------
[01:03:05] 
[01:03:05] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:03:05] 
[01:03:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:03:05] 
[01:03:05] 
[01:03:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:05] 
[01:03:05] 
[01:03:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:05] Build completed unsuccessfully in 0:03:57
[01:03:05] Build completed unsuccessfully in 0:03:57
[01:03:05] Makefile:48: recipe for target 'check' failed
[01:03:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:044f8936
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 12:26:38 UTC 2019
---
travis_time:end:046af428:start=1551183999860482742,finish=1551183999865007854,duration=4525112
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01933224
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0361d15a
travis_time:start:0361d15a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:143d35f8
$ dmesg | grep -i kill
