plain
travis_time:end:136762c2:start=1559510365841234201,finish=1559510457475205091,duration=91633970890
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
[01:08:40] 
[01:08:40] running 50 tests
[01:09:02] ......F.......................F...................
[01:09:02] failures:
[01:09:02] 
[01:09:02] ---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
[01:09:02] ---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
[01:09:02] thread '[mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:09:02] Expected Line: "     _3 = _4;"
[01:09:02] Test Name: rustc.main.ConstProp.after.mir
[01:09:02] ... (elided)
[01:09:02]  bb0: {
[01:09:02] ... (elided)
[01:09:02]      _3 = _4;
[01:09:02]      _3 = _4;
[01:09:02]      _2 = move _3 as *const i32 (Misc);
[01:09:02] ... (elided)
[01:09:02]      _1 = move _2 as usize (Misc);
[01:09:02] ... (elided)
[01:09:02]      _6 = _1;
[01:09:02]      _5 = const read(move _6) -> bb1;
[01:09:02] Actual:
[01:09:02] Actual:
[01:09:02] fn  main() -> () {
[01:09:02]     let mut _0: ();
[01:09:02]     let _1: usize;
[01:09:02]     let mut _2: *const i32;
[01:09:02]     let mut _3: &i32;
[01:09:02]     let mut _4: &i32;
[01:09:02]     let mut _5: ();
[01:09:02]     let mut _6: usize;
[01:09:02]     scope 1 {
[01:09:02]     bb0: {
[01:09:02]     bb0: {
[01:09:02]         StorageLive(_1);
[01:09:02]         StorageLive(_2);
[01:09:02]         StorageLive(_3);
[01:09:02]         StorageLive(_4);
[01:09:02]         _4 = const ByRef(AllocId(1).0x0, Allocation { bytes: [1, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [15], len: Size { raw: 4 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) : &i32;
[01:09:02]         _3 = const ByRef(AllocId(1).0x0, Allocation { bytes: [1, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [15], len: Size { raw: 4 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) : &i32;
[01:09:02]         _2 = const ByRef(AllocId(1).0x0, Allocation { bytes: [1, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [15], len: Size { raw: 4 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) : *const i32;
[01:09:02]         StorageDead(_3);
[01:09:02]         _1 = move _2 as usize (Misc);
[01:09:02]         StorageDead(_2);
[01:09:02]         StorageDead(_4);
[01:09:02]         StorageLive(_6);
[01:09:02]         _6 = _1;
[01:09:02]         _5 = const read(move _6) -> bb1;
[01:09:02]     bb1: {
[01:09:02]     bb1: {
[01:09:02]         StorageDead(_6);
[01:09:02]         _0 = ();
[01:09:02]         StorageDead(_1);
[01:09:02]         return;
[01:09:02]     }
[01:09:02]     bb2 (cleanup): {
[01:09:02]         resume;
[01:09:02] }', src/tools/compiletest/src/runtest.rs:3196:13
[01:09:02] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:09:02] 
[01:09:02] ---- [mir-opt] mir-opt/match-arm-scopes.rs stdout ----
---
[01:09:02] test result: FAILED. 48 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:09:02] 
[01:09:02] 
[01:09:02] 
[01:09:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:02] 
[01:09:02] 
[01:09:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:02] Build completed unsuccessfully in 1:05:02
---
travis_time:end:22590480:start=1559514612904673442,finish=1559514612910351140,duration=5677698
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0167f634
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.20878.!checkout!obj!build!x86_64-unknown-linux-gnu!test!mir-opt!match-arm-scopes!a
[New LWP 20878]
warning: Could not load shared library symbols for 6 libraries, e.g. /lib/x86_64-linux-gnu/libgcc_s.so.1.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes/a'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x000055835b1bbe40 in match_arm_scopes::main::h877b300a78133b47 ()
#0  0x000055835b1bbe40 in match_arm_scopes::main::h877b300a78133b47 ()
#1  0x000055835b1bc023 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he79fb1b7bf161dd1 ()
#2  0x00007f25446b0873 in std::panicking::try::do_call::h706902bbfd5c690f () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-a61f12a1bce0d2db.so
#3  0x00007f25446c4f4a in __rust_maybe_catch_panic () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-a61f12a1bce0d2db.so
#4  0x00007f25446b1550 in std::rt::lang_start_internal::hbed100108adadca4 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-a61f12a1bce0d2db.so
#5  0x000055835b1bc015 in main ()
travis_time:end:0167f634:start=1559514612914898721,finish=1559514613146766990,duration=231868269
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:131b1ab7
travis_time:start:131b1ab7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown
