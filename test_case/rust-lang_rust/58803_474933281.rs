plain
travis_time:end:1a938dc9:start=1553097330414502875,finish=1553097454419417380,duration=124004914505
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:03:32] .................................................................................................... 1800/2959
[01:03:41] .................................................................................................... 1900/2959
[01:03:53] .........i......................................................................i................... 2000/2959
[01:04:14] .................................................................................................... 2100/2959
[01:04:33] ..F................................................................................................. 2200/2959
[01:04:54] ................................................ii.................................................. 2400/2959
[01:05:07] .................................................................................................... 2500/2959
[01:05:26] .................................................................................................... 2600/2959
[01:05:43] .................................................................................................... 2700/2959
---
[01:06:09] 
[01:06:09] ------------------------------------------
[01:06:09] stderr:
[01:06:09] ------------------------------------------
[01:06:09] thread 'main' panicked at 'Failed to create empty file `a`: Os { code: 26, kind: Other, message: "Text file busy" }', src/libcore/result.rs:997:5
[01:06:09] 
[01:06:09] ------------------------------------------
[01:06:09] 
[01:06:09] thread '[run-pass] run-pass/paths-containing-nul.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:06:09] 
[01:06:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:06:09] 
[01:06:09] 
[01:06:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:09] 
[01:06:09] 
[01:06:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:09] Build completed unsuccessfully in 0:09:08
[01:06:09] Build completed unsuccessfully in 0:09:08
[01:06:09] make: *** [check] Error 1
[01:06:09] Makefile:48: recipe for target 'check' failed
bj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02b1b03c
travis_time:start:02b1b03c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2742495d
$ dmesg | grep -i kill
