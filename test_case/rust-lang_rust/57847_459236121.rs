plain
travis_time:end:008ec2db:start=1548913547281438917,finish=1548913620486458746,duration=73205019829
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:46] .................................................................................................... 4200/5355
[01:03:51] .................................................................................................... 4300/5355
[01:03:54] .................................................................................................... 4400/5355
[01:03:57] .................................................................................................... 4500/5355
[01:04:02] ......i........................................................................F.................... 4600/5355
[01:04:10] .................................................................................................... 4800/5355
[01:04:14] .................................................................................................... 4900/5355
[01:04:17] .................................................................................................... 5000/5355
[01:04:21] .................................................................................................... 5100/5355
---
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:04:28]   left: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24", "}", ":29] b = Point {", "    x: 42,", "    y: 24", "}", ":37]", ":41] &a = NoCopy(", "    1337", ")", ":41] dbg!(& a) = NoCopy(", "    1337", ")", ":46] f(&42) = 42", "before", ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331"]`,
[01:04:28]  right: `[":21] Unit = Unit", ":22] a = Unit", ":28] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24", "}", ":29] b = Point {", "    x: 42,", "    y: 24", "}", ":37]", ":40] &a = NoCopy(", "    1337", ")", ":40] dbg!(& a) = NoCopy(", "    1337", ")", ":45] f(&42) = 42", "before", ":50] { foo += 1; eprintln!(\"before\"); 7331 } = 7331"]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:60:5
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3291:9
---
[01:04:28] 
[01:04:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:04:28] 
[01:04:28] 
[01:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:28] 
[01:04:28] 
[01:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:28] Build completed unsuccessfully in 0:04:04
[01:04:28] Build completed unsuccessfully in 0:04:04
[01:04:28] make: *** [check] Error 1
[01:04:28] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d289130
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 06:51:37 UTC 2019
---
travis_time:end:007dc580:start=1548917498628774313,finish=1548917498633131201,duration=4356888
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18a81c5b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!che
