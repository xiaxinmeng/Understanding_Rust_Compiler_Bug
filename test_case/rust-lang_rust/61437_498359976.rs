plain
travis_time:end:0aae1fdc:start=1559580181110513026,finish=1559580272626839392,duration=91516326366
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
[01:08:48] 
[01:08:48] running 51 tests
[01:09:11] ......FF...........................................
[01:09:11] 
[01:09:11] ---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
[01:09:11] ---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
[01:09:11] thread '[mir-opt] mir-opt/const_prop/indirect.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:09:11] Expected Line: "    _2 = const 2u32 as u8 (Misc);"
[01:09:11] Test Name: rustc.main.ConstProp.after.mir
[01:09:11] ... (elided)
[01:09:11] bb0: {
[01:09:11] ... (elided)
[01:09:11] ... (elided)
[01:09:11]     _2 = const 2u32 as u8 (Misc);
[01:09:11]     _3 = (const 3u8, const false);
[01:09:11]     assert(!const false, "attempt to add with overflow") -> bb1;
[01:09:11] }
[01:09:11] Actual:
[01:09:11] fn  main() -> () {
[01:09:11]     let mut _0: ();
[01:09:11]     let _1: u8;
[01:09:11]     let mut _2: u8;
[01:09:11]     let mut _3: (u8, bool);
[01:09:11]     scope 1 {
[01:09:11]     bb0: {
[01:09:11]     bb0: {
[01:09:11]         StorageLive(_1);
[01:09:11]         StorageLive(_2);
[01:09:11]         _2 = const 2u8;
[01:09:11]         _3 = (const 3u8, const false);
[01:09:11]         assert(!const false, "attempt to add with overflow") -> bb1;
[01:09:11]     bb1: {
[01:09:11]     bb1: {
[01:09:11]         _1 = move (_3.0: u8);
[01:09:11]         StorageDead(_2);
[01:09:11]         _0 = ();
[01:09:11]         StorageDead(_1);
[01:09:11]         return;
[01:09:11]     }
[01:09:11]     bb2 (cleanup): {
[01:09:11]         resume;
[01:09:11] }', src/tools/compiletest/src/runtest.rs:3196:13
[01:09:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:09:11] 
[01:09:11] ---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
[01:09:11] ---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
[01:09:11] thread '[mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:09:11] Expected Line: "     _3 = _4;"
[01:09:11] Test Name: rustc.main.ConstProp.after.mir
[01:09:11] ... (elided)
[01:09:11]  bb0: {
[01:09:11] ... (elided)
[01:09:11]      _3 = _4;
[01:09:11]      _3 = _4;
[01:09:11]      _2 = move _3 as *const i32 (Misc);
[01:09:11] ... (elided)
[01:09:11]      _1 = move _2 as usize (Misc);
[01:09:11] ... (elided)
[01:09:11]      _6 = _1;
[01:09:11]      _5 = const read(move _6) -> bb1;
[01:09:11] Actual:
[01:09:11] Actual:
[01:09:11] fn  main() -> () {
[01:09:11]     let mut _0: ();
[01:09:11]     let _1: usize;
[01:09:11]     let mut _2: *const i32;
[01:09:11]     let mut _3: &i32;
[01:09:11]     let mut _4: &i32;
[01:09:11]     let mut _5: ();
[01:09:11]     let mut _6: usize;
[01:09:11]     scope 1 {
[01:09:11]     bb0: {
[01:09:11]     bb0: {
[01:09:11]         StorageLive(_1);
[01:09:11]         StorageLive(_2);
[01:09:11]         StorageLive(_3);
[01:09:11]         StorageLive(_4);
[01:09:11]         _4 = const Scalar(AllocId(1).0x0) : &i32;
[01:09:11]         _3 = const Scalar(AllocId(1).0x0) : &i32;
[01:09:11]         _2 = const Scalar(AllocId(1).0x0) : *const i32;
[01:09:11]         StorageDead(_3);
[01:09:11]         _1 = move _2 as usize (Misc);
[01:09:11]         StorageDead(_2);
[01:09:11]         StorageDead(_4);
[01:09:11]         StorageLive(_6);
[01:09:11]         _6 = _1;
[01:09:11]         _5 = const read(move _6) -> bb1;
[01:09:11]     bb1: {
[01:09:11]     bb1: {
[01:09:11]         StorageDead(_6);
[01:09:11]         _0 = ();
[01:09:11]         StorageDead(_1);
[01:09:11]         return;
[01:09:11]     }
[01:09:11]     bb2 (cleanup): {
[01:09:11]         resume;
[01:09:11] }', src/tools/compiletest/src/runtest.rs:3196:13
[01:09:11] 
[01:09:11] 
[01:09:11] failures:
---
[01:09:11] 
[01:09:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:09:11] 
[01:09:11] 
[01:09:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:11] 
[01:09:11] 
[01:09:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:11] Build completed unsuccessfully in 1:05:10
