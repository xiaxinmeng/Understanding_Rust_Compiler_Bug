
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:49:44] 
[00:49:44] running 30 tests
[00:49:48] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:323:21
[00:49:48] .........................F..FF
[00:49:48] failures:
[00:49:48] 
[00:49:48] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[00:49:48] 	thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'ran out of mir dump output to match against.
[00:49:48] Did not find expected line: "        Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(5) => validate_1/8cd878b::{{impl}}[0]::foo[0] }, BrAnon(0)) Test, _2: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(5) => validate_1/8cd878b::{{impl}}[0]::foo[0] }, BrAnon(1)) mut i32]);"
[00:49:48] Expected:
[00:49:48]     bb0: {
[00:49:48]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(5) => validate_1/8cd878b::{{impl}}[0]::foo[0] }, BrAnon(0)) Test, _2: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(5) => validate_1/8cd878b::{{impl}}[0]::foo[0] }, BrAnon(1)) mut i32]);
[00:49:48]         return;
[00:49:48]     }
[00:49:48] Actual:
[00:49:48] fn <impl at /checkout/src/test/mir-opt/validate_1.rs:16:1: 19:2>::foo(_1: &ReErased Test, _2: &ReErased mut i32) -> () {
[00:49:48]     let mut _0: ();
[00:49:48]     scope 1 {
[00:49:48]         let _3: &ReErased Test;
[00:49:48]         let _4: &ReErased mut i32;
[00:49:48]     }
[00:49:48]     bb0: {
[00:49:48]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(0:5) => validate_1/8cd878b::{{impl}}[0]::foo[0] }, BrAnon(0)) Test, _2: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(0:5) => validate_1/8cd878b::{{impl}}[0]::foo[0] }, BrAnon(1)) mut i32]);
[00:49:48]         StorageLive(_3);
[00:49:48]         _3 = _1;
[00:49:48]         StorageLive(_4);
[00:49:48]         _4 = _2;
[00:49:48]         _0 = ();
[00:49:48]         StorageDead(_4);
[00:49:48]         StorageDead(_3);
[00:49:48]         return;
[00:49:48]     }
[00:49:48] }', /checkout/src/tools/compiletest/src/runtest.rs:2315:16
[00:49:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:48] 
[00:49:48] ---- [mir-opt] mir-opt/validate_5.rs stdout ----
[00:49:48] 	thread '[mir-opt] mir-opt/validate_5.rs' panicked at 'ran out of mir dump output to match against.
[00:49:48] Did not find expected line: "        Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(4) => validate_5/8cd878b::test[0] }, BrAnon(0)) mut i32]);"
[00:49:48] Expected:
[00:49:48] fn test(_1: &ReErased mut i32) -> () {
[00:49:48]     bb0: {
[00:49:48]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(4) => validate_5/8cd878b::test[0] }, BrAnon(0)) mut i32]);
[00:49:48]         Validate(Release, [_3: bool, _4: *mut i32]);
[00:49:48]         _3 = const write_42(_4) -> bb1;
[00:49:48]     }
[00:49:48] }
[00:49:48] Actual:
[00:49:48] fn test(_1: &ReErased mut i32) -> () {
[00:49:48]     let mut _0: ();
[00:49:48]     scope 1 {
[00:49:48]         let _2: &ReErased mut i32;
[00:49:48]     }
[00:49:48]     let mut _3: bool;
[00:49:48]     let mut _4: *mut i32;
[00:49:48]     let mut _5: &ReErased mut i32;
[00:49:48]     bb0: {
[00:49:48]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(0:4) => validate_5/8cd878b::test[0] }, BrAnon(0)) mut i32]);
[00:49:48]         StorageLive(_2);
[00:49:48]         _2 = _1;
[00:49:48]         StorageLive(_4);
[00:49:48]         StorageLive(_5);
[00:49:48]         Validate(Suspend(ReScope(Node(ItemLocalId(2)))), [(*_2): i32]);
[00:49:48]         _5 = &ReErased mut (*_2);
[00:49:48]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(2)))]);
[00:49:48]         _4 = _5 as *mut i32 (Misc);
[00:49:48]         StorageDead(_5);
[00:49:48]         EndRegion(ReScope(Node(ItemLocalId(2))));
[00:49:48]         Validate(Release, [_3: bool, _4: *mut i32]);
[00:49:48]         _3 = const write_42(_4) -> bb1;
[00:49:48]     }
[00:49:48]     bb1: {
[00:49:48]         Validate(Acquire, [_3: bool]);
[00:49:48]         StorageDead(_4);
[00:49:48]         _0 = ();
[00:49:48]         StorageDead(_2);
[00:49:48]         return;
[00:49:48]     }
[00:49:48] }', /checkout/src/tools/compiletest/src/runtest.rs:2315:16
[00:49:48] 
[00:49:48] ---- [mir-opt] mir-opt/validate_4.rs stdout ----
[00:49:48] 	thread '[mir-opt] mir-opt/validate_4.rs' panicked at 'ran out of mir dump output to match against.
[00:49:48] Did not find expected line: "        Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(2147483659) => validate_4/8cd878b::write_42[0]::{{closure}}[0] }, \"BrEnv\") [closure@NodeId(22)], _2: *mut i32]);"
[00:49:48] Expected:
[00:49:48] fn write_42::{{closure}}(_1: &ReErased [closure@NodeId(22)], _2: *mut i32) -> () {
[00:49:48]     bb0: {
[00:49:48]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(2147483659) => validate_4/8cd878b::write_42[0]::{{closure}}[0] }, "BrEnv") [closure@NodeId(22)], _2: *mut i32]);
[00:49:48]         Validate(Release, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(2147483659) => validate_4/8cd878b::write_42[0]::{{closure}}[0] }, "BrEnv") [closure@NodeId(22)], _2: *mut i32]);
[00:49:48]         StorageLive(_3);
[00:49:48]         _3 = _2;
[00:49:48]         (*_3) = const 23i32;
[00:49:48]         StorageDead(_3);
[00:49:48]         return;
[00:49:48]     }
[00:49:48] }
[00:49:48] Actual:
[00:49:48] fn write_42::{{closure}}(_1: &ReErased [closure@NodeId(22)], _2: *mut i32) -> () {
[00:49:48]     let mut _0: ();
[00:49:48]     scope 1 {
[00:49:48]         let _3: *mut i32;
[00:49:48]     }
[00:49:48]     bb0: {
[00:49:48]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(1:11) => validate_4/8cd878b::write_42[0]::{{closure}}[0] }, "BrEnv") [closure@NodeId(22)], _2: *mut i32]);
[00:49:48]         Validate(Release, [_1: &ReFree(DefId { krate: CrateNum(0), node: DefIndex(1:11) => validate_4/8cd878b::write_42[0]::{{closure}}[0] }, "BrEnv") [closure@NodeId(22)], _2: *mut i32]);
[00:49:48]         StorageLive(_3);
[00:49:48]         _3 = _2;
[00:49:48]         (*_3) = const 23i32;
[00:49:48]         StorageDead(_3);
[00:49:48]         return;
[00:49:48]     }
[00:49:48] }', /checkout/src/tools/compiletest/src/runtest.rs:2315:16
[00:49:48] 
[00:49:48] 
[00:49:48] failures:
[00:49:48]     [mir-opt] mir-opt/validate_1.rs
[00:49:48]     [mir-opt] mir-opt/validate_4.rs
[00:49:48]     [mir-opt] mir-opt/validate_5.rs
[00:49:48] 
[00:49:48] test result: FAILED. 27 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[00:49:48] 
[00:49:48] 
[00:49:48] 
[00:49:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.7/bin/FileCheck" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.7.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:48] expected success, got: exit code: 101
[00:49:48] 
[00:49:48] 
[00:49:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:48] Build completed unsuccessfully in 0:11:48
[00:49:48] Makefile:52: recipe for target 'check' failed
[00:49:48] make: *** [check] Error 1
