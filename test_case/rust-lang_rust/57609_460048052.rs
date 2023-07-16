plain
travis_time:end:029439f8:start=1549193236305087097,finish=1549193238395685894,duration=2090598797
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:18] 
[01:11:18] running 129 tests
[01:11:22] i..ii...iii..iiii.....i.................i..i................i..F..i.........ii...i..i.ii............ 100/129
[01:11:23] ..i........ii.i.....iii......
[01:11:23] failures:
[01:11:23] 
[01:11:23] ---- [codegen] codegen/match.rs stdout ----
[01:11:23] 
[01:11:23] 
[01:11:23] error: verification with 'FileCheck' failed
[01:11:23] status: exit code: 1
[01:11:23] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match/match.ll" "/checkout/src/test/codegen/match.rs"
[01:11:23] ------------------------------------------
[01:11:23] 
[01:11:23] ------------------------------------------
[01:11:23] stderr:
[01:11:23] stderr:
[01:11:23] ------------------------------------------
[01:11:23] /checkout/src/test/codegen/match.rs:21:11: error: expected string not found in input
[01:11:23] // CHECK: [[OTHERWISE]]:
[01:11:23]           ^
[01:11:23] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match/match.ll:32:1: note: scanning from here
[01:11:23] }
[01:11:23] ^
[01:11:23] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match/match.ll:32:1: note: with variable "OTHERWISE" equal to "bb1"
[01:11:23] }
[01:11:23] 
[01:11:23] ------------------------------------------
[01:11:23] 
[01:11:23] thread '[codegen] codegen/match.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:11:23] test result: FAILED. 100 passed; 1 failed; 28 ignored; 0 measured; 0 filtered out
[01:11:23] 
[01:11:23] 
[01:11:23] 
[01:11:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:23] 
[01:11:23] 
[01:11:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:23] Build completed unsuccessfully in 0:11:17
[01:11:23] Build completed unsuccessfully in 0:11:17
[01:11:23] Makefile:48: recipe for target 'check' failed
[01:11:23] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f944ff1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 12:38:51 UTC 2019
---
travis_time:end:21725f1f:start=1549197533432636816,finish=1549197533438344774,duration=5707958
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05013082
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16c49724
travis_time:start:16c49724
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b82e71c
$ dmesg | grep -i kill
