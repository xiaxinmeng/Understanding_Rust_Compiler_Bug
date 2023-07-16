plain
travis_time:end:1fc22dd5:start=1559612360002673589,finish=1559612448879030645,duration=88876357056
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:10] 
[01:04:10] running 51 tests
[01:04:31] ......F............................................
[01:04:31] 
[01:04:31] ---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
[01:04:31] ---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
[01:04:31] thread '[mir-opt] mir-opt/const_prop/indirect.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:04:31] Expected Line: "    _2 = const 2u8 (Misc);"
[01:04:31] Test Name: rustc.main.ConstProp.after.mir
[01:04:31] ... (elided)
[01:04:31] bb0: {
[01:04:31] ... (elided)
[01:04:31] ... (elided)
[01:04:31]     _2 = const 2u8 (Misc);
[01:04:31]     _3 = (const 3u8, const false);
[01:04:31]     assert(!const false, "attempt to add with overflow") -> bb1;
[01:04:31] }
[01:04:31] Actual:
[01:04:31] fn  main() -> () {
[01:04:31]     let mut _0: ();
[01:04:31]     let _1: u8;
[01:04:31]     let mut _2: u8;
[01:04:31]     let mut _2: u8;
[01:04:31]     let mut _3: (u8, bool);
[01:04:31]     scope 1 {
[01:04:31]     bb0: {
[01:04:31]     bb0: {
[01:04:31]         StorageLive(_1);
[01:04:31]         StorageLive(_2);
[01:04:31]         _2 = const 2u8;
[01:04:31]         _3 = (const 3u8, const false);
[01:04:31]         assert(!const false, "attempt to add with overflow") -> bb1;
[01:04:31]     bb1: {
[01:04:31]     bb1: {
[01:04:31]         _1 = move (_3.0: u8);
[01:04:31]         StorageDead(_2);
[01:04:31]         _0 = ();
[01:04:31]         StorageDead(_1);
[01:04:31]         return;
[01:04:31]     }
[01:04:31]     bb2 (cleanup): {
[01:04:31]         resume;
[01:04:31] }', src/tools/compiletest/src/runtest.rs:3196:13
[01:04:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:31] 
[01:04:31] 
[01:04:31] 
[01:04:31] failures:
[01:04:31]     [mir-opt] mir-opt/const_prop/indirect.rs
[01:04:31] 
[01:04:31] test result: FAILED. 50 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:31] 
[01:04:31] 
[01:04:31] 
[01:04:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:31] 
[01:04:31] 
[01:04:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:31] Build completed unsuccessfully in 0:59:18
---
travis_time:end:20745522:start=1559616331415453803,finish=1559616331470368262,duration=54914459
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:117e2aa6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
