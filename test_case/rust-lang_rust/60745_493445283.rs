plain
travis_time:end:099bb986:start=1558093176546806059,finish=1558093264702666206,duration=88155860147
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:57] 
[01:20:57] running 48 tests
[01:21:18] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:21:18] ......................................F.........
[01:21:18] 
[01:21:18] ---- [mir-opt] mir-opt/simplify_if.rs stdout ----
[01:21:18] ---- [mir-opt] mir-opt/simplify_if.rs stdout ----
[01:21:18] thread '[mir-opt] mir-opt/simplify_if.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:21:18] Expected Line: "    switchInt(const false) -> [false: bb3, otherwise: bb1];"
[01:21:18] Test Name: rustc.main.SimplifyBranches-after-copy-prop.before.mir
[01:21:18] ... (elided)
[01:21:18] bb0: {
[01:21:18] ... (elided)
[01:21:18] ... (elided)
[01:21:18]     switchInt(const false) -> [false: bb3, otherwise: bb1];
[01:21:18] }
[01:21:18] Actual:
[01:21:18] | User Type Annotations
[01:21:18] | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2:6704 ~ core[6ea2]::fmt[0]::{{impl}}[2]::new_v1[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0))], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:6702 ~ core[6ea2]::fmt[0]::{{impl}}[2]), self_ty: std::fmt::Arguments<'_> }) }) } at <::std::macros::println macros>:2:29: 2:63
[01:21:18] |
[01:21:18] fn  main() -> () {
[01:21:18]     let mut _0: ();
[01:21:18]     let mut _1: bool;
[01:21:18]     let mut _2: ();
[01:21:18]     let mut _3: ();
[01:21:18]     let mut _4: std::fmt::Arguments;
[01:21:18]     let mut _5: &[&str];
[01:21:18]     let mut _6: &[&str; 1];
[01:21:18]     let mut _7: &[&str; 1];
[01:21:18]     let _8: [&str; 1];
[01:21:18]     let mut _9: &str;
[01:21:18]     let mut _10: &str;
[01:21:18]     let mut _11: &[std::fmt::ArgumentV1];
[01:21:18]     let mut _12: &[std::fmt::ArgumentV1; 0];
[01:21:18]     let mut _13: &[std::fmt::ArgumentV1; 0];
[01:21:18]     let _14: [std::fmt::ArgumentV1; 0];
[01:21:18]     let mut _15: ();
[01:21:18]     scope 1 {
[01:21:18]         let mut _16: &[&str];
[01:21:18]         let mut _17: std::option::Option<&[std::fmt::rt::v1::Argument]>;
[01:21:18]         let mut _18: &[std::fmt::ArgumentV1];
[01:21:18]     bb0: {
[01:21:18]     bb0: {
[01:21:18]         StorageLive(_1);
[01:21:18]         _1 = const false;
[01:21:18]         goto -> bb3;
[01:21:18]     bb1: {
[01:21:18]     bb1: {
[01:21:18]         StorageLive(_4);
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         _7 = &(promoted[1]: [&str; 1]);
[01:21:18]         nop;
[01:21:18]         _5 = move _7 as &[&str] (Pointer(Unsize));
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         StorageLive(_15);
[01:21:18]         nop;
[01:21:18]         _13 = &(promoted[0]: [std::fmt::ArgumentV1; 0]);
[01:21:18]         nop;
[01:21:18]         _11 = move _13 as &[std::fmt::ArgumentV1] (Pointer(Unsize));
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         StorageLive(_17);
[01:21:18]         discriminant(_17) = 0;
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         (_4.0: &[&str]) = move _5;
[01:21:18]         (_4.1: std::option::Option<&[std::fmt::rt::v1::Argument]>) = move _17;
[01:21:18]         (_4.2: &[std::fmt::ArgumentV1]) = move _11;
[01:21:18]         nop;
[01:21:18]         StorageDead(_17);
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         _3 = const std::io::_print(move _4) -> bb2;
[01:21:18]     bb2: {
[01:21:18]     bb2: {
[01:21:18]         StorageDead(_4);
[01:21:18]         nop;
[01:21:18]         StorageDead(_15);
[01:21:18]         nop;
[01:21:18]         nop;
[01:21:18]         goto -> bb4;
[01:21:18]     }
[01:21:18]     }
[01:21:18]     bb3: {
[01:21:18]         nop;
[01:21:18]         goto -> bb4;
[01:21:18]     }
[01:21:18]     bb4: {
[01:21:18]         StorageDead(_1);
[01:21:18]         return;
[01:21:18] }', src/tools/compiletest/src/runtest.rs:3102:13
[01:21:18] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:18] 
[01:21:18] 
[01:21:18] 
[01:21:18] failures:
[01:21:18]     [mir-opt] mir-opt/simplify_if.rs
[01:21:18] 
[01:21:18] test result: FAILED. 47 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:18] 
[01:21:18] 
[01:21:18] 
[01:21:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:18] 
[01:21:18] 
[01:21:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:18] Build completed unsuccessfully in 0:12:32
[01:21:18] Build completed unsuccessfully in 0:12:32
[01:21:18] make: *** [check] Error 1
[01:21:18] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:134d5ff0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 13:02:32 UTC 2019
---
travis_time:end:0148bb80:start=1558098154284330538,finish=1558098154289739543,duration=5409005
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b3f9412
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e87a61a
travis_time:start:1e87a61a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:182c3d90
$ dmesg | grep -i kill
