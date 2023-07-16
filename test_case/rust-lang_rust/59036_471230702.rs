plain
travis_time:end:02feb64e:start=1552168304930864865,finish=1552168379302959201,duration=74372094336
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:18:51] ...................................F...
[01:18:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:18:51] failures:
[01:18:51] 
[01:18:51] ---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
[01:18:51] thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'failed to exec `"/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unusual-item-types/a"`: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }', src/libcore/result.rs:997:5
[01:18:51] 
[01:18:51] 
[01:18:51] failures:
[01:18:51]     [mir-opt] mir-opt/unusual-item-types.rs
[01:18:51]     [mir-opt] mir-opt/unusual-item-types.rs
[01:18:51] 
[01:18:51] test result: FAILED. 38 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:51] 
[01:18:51] 
[01:18:51] 
[01:18:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:51] 
[01:18:51] 
[01:18:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:51] Build completed unsuccessfully in 0:11:22
[01:18:51] Build completed unsuccessfully in 0:11:22
[01:18:51] make: *** [check] Error 1
[01:18:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06526bfa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  9 23:12:00 UTC 2019
