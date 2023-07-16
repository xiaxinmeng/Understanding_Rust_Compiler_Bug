plain
travis_time:end:0e3f8ca4:start=1550143897751002962,finish=1550144212822620668,duration=315071617706
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:07:43] .................................................................................i.................. 200/2949
[01:07:51] .................................................................................................... 300/2949
[01:08:02] .................................................................................................... 400/2949
[01:08:11] .................................................................................................... 500/2949
[01:08:23] ...F...F............................................................................................ 600/2949
[01:08:49] .................................................................................................... 800/2949
[01:08:58] .................................................................................................... 900/2949
[01:09:15] .................................................................................................... 1000/2949
[01:09:27] .................................................................................................... 1100/2949
---
[01:13:53] ------------------------------------------
[01:13:53] stderr:
[01:13:53] ------------------------------------------
[01:13:53] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:13:53]   left: `0x5616ededb008`,
[01:13:53]  right: `0x5616ededb048`', /checkout/src/test/run-pass/consts/const-region-ptrs-noncopy.rs:11:5
[01:13:53] 
[01:13:53] ------------------------------------------
[01:13:53] 
[01:13:53] thread '[run-pass] run-pass/consts/const-region-ptrs-noncopy.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:13:53] ------------------------------------------
[01:13:53] stderr:
[01:13:53] ------------------------------------------
[01:13:53] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:13:53]   left: `0x55acf3872c6d`,
[01:13:53]  right: `0x55acf3a74008`', /checkout/src/test/run-pass/consts/const-str-ptr.rs:12:9
[01:13:53] 
[01:13:53] ------------------------------------------
[01:13:53] 
[01:13:53] thread '[run-pass] run-pass/consts/const-str-ptr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:13:53] 
[01:13:53] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:53] 
[01:13:53] 
[01:13:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:53] 
[01:13:53] 
[01:13:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:53] Build completed unsuccessfully in 0:10:47
[01:13:53] Build completed unsuccessfully in 0:10:47
[01:13:53] make: *** [check] Error 1
[01:13:53] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01da6c66
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 12:50:56 UTC 2019
---
travis_time:end:189c1932:start=1550148657458448616,finish=1550148657464234182,duration=5785566
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06fde980
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e89a44f
travis_time:start:1e89a44f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:065fd4e2
$ dmesg | grep -i kill
