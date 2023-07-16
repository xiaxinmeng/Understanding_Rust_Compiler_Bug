plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:09] 
[00:55:09] running 51 tests
[00:55:25] ERROR 2018-07-13T22:50:22Z: compiletest::runtest: Some("        Validate(Acquire, [_1: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(0)) Test, _2: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(1)) mut i32]);")
[00:55:27] ERROR 2018-07-13T22:50:24Z: compiletest::runtest: None
[00:55:28] ERROR 2018-07-13T22:50:25Z: compiletest::runtest: None
[00:55:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:30] ...........................................F..F.F..
[00:55:30] 
[00:55:30] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[00:55:30] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[00:55:30] thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:30] Current block:         Validate(Acquire, [_1: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(0)) Test, _2: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(1)) mut i32]);
[00:55:30] Actual Line: "        _0 = ();"
[00:55:30] Expected Line: "        Validate(Acquire, [_1: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(0)) Test, _2: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(1)) mut i32]);"
[00:55:30] Test Name: rustc.{{impl}}-foo.EraseRegions.after.mir
[00:55:30] Expected:
[00:55:30] ... (elided)
[00:55:30]     bb0: {
[00:55:30]         Validate(Acquire, [_1: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(0)) Test, _2: &ReFree(DefId(0/0:5 ~ validate_1[317d]::{{impl}}[0]::foo[0]), BrAnon(1)) mut i32]);
[00:55:30] ... (elided)
[00:55:30]         return;
[00:55:30] Actual:
[00:55:30] Actual:
[00:55:30] fn <impl at /checkout/src/test/mir-opt/validate_1.rs:16:1: 19:2>::foo(_1: &ReErased Test, _2: &ReErased mut i32) -> (){
[00:55:30]     let mut _0: ();
[00:55:30]     bb0: {                              
[00:55:30]         _0 = ();
[00:55:30]         return;
[00:55:30] }', tools/compiletest/src/runtest.rs:2815:13
[00:55:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:30] 
[00:55:30] ---- [mir-opt] mir-opt/validate_4.rs stdout ----
[00:55:30] ---- [mir-opt] mir-opt/validate_4.rs stdout ----
[00:55:30] thread '[mir-opt] mir-opt/validate_4.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:30] Current block: None
[00:55:30] Actual Line: "        StorageLive(_2);"
[00:55:30] Expected Line: "        Validate(Acquire, [_1: *mut i32]);"
[00:55:30] Test Name: rustc.write_42.EraseRegions.after.mir
[00:55:30] Expected:
[00:55:30] ... (elided)
[00:55:30] fn write_42(_1: *mut i32) -> bool {
[00:55:30] ... (elided)
[00:55:30]     bb0: {
[00:55:30]         Validate(Acquire, [_1: *mut i32]);
[00:55:30]         Validate(Release, [_1: *mut i32]);
[00:55:30] ... (elided)
[00:55:30]         return;
[00:55:30] }
[00:55:30] Actual:
[00:55:30] Actual:
[00:55:30] fn write_42(_1: *mut i32) -> bool{
[00:55:30]     let mut _0: bool;
[00:55:30]     scope 1 {
[00:55:30]     scope 2 {
[00:55:30]     scope 2 {
[00:55:30]         let _2: [closure@NodeId(22)];
[00:55:30]     }
[00:55:30]     let mut _3: ();
[00:55:30]     let mut _4: &ReErased [closure@NodeId(22)];
[00:55:30]     let mut _5: (*mut i32,);
[00:55:30]     let mut _6: *mut i32;
[00:55:30]     bb0: {                              
[00:55:30]         StorageLive(_2);
[00:55:30]         _2 = [closure@NodeId(22)];
[00:55:30]         StorageLive(_4);
[00:55:30]         _4 = &ReErased _2;
[00:55:30]         StorageLive(_5);
[00:55:30]         StorageLive(_6);
[00:55:30]         _6 = _1;
[00:55:30]         _5 = (move _6,);
[00:55:30]         _3 = const std::ops::Fn::call(move _4, move _5) -> bb1;
[00:55:30]     }
[00:55:30]     bb1: {                              
[00:55:30]         Validate(Acquire, [_3: ()]);
[00:55:30]         Validate(Release, [_3: ()]);
[00:55:30]         EndRegion(ReScope(Node(ItemLocalId(15))));
[00:55:30]         StorageDead(_5);
[00:55:30]         StorageDead(_6);
[00:55:30]         StorageDead(_4);
[00:55:30]         (*_1) = const 42i32;
[00:55:30]         _0 = const true;
[00:55:30]         StorageDead(_2);
[00:55:30]         return;
[00:55:30] }', tools/compiletest/src/runtest.rs:2815:13
[00:55:30] 
[00:55:30] ---- [mir-opt] mir-opt/validate_5.rs stdout ----
[00:55:30] ---- [mir-opt] mir-opt/validate_5.rs stdout ----
[00:55:30] thread '[mir-opt] mir-opt/validate_5.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:30] Current block: None
[00:55:30] Actual Line: "        StorageLive(_3);"
[00:55:30] Expected Line: "        Validate(Acquire, [_1: &ReFree(DefId(0/0:4 ~ validate_5[317d]::test[0]), BrAnon(0)) mut i32]);"
[00:55:30] Test Name: rustc.test.EraseRegions.after.mir
[00:55:30] Expected:
[00:55:30] ... (elided)
[00:55:30] fn test(_1: &ReErased mut i32) -> () {
[00:55:30] ... (elided)
[00:55:30]     bb0: {
[00:55:30]         Validate(Acquire, [_1: &ReFree(DefId(0/0:4 ~ validate_5[317d]::test[0]), BrAnon(0)) mut i32]);
[00:55:30] ... (elided)
[00:55:30]         Validate(Release, [_2: bool, _3: *mut i32]);
[00:55:30]         _2 = const write_42(move _3) -> bb1;
[00:55:30]     }
[00:55:30] ... (elided)
[00:55:30] }
[00:55:30] Actual:
[00:55:30] fn test(_1: &ReErased mut i32) -> (){
[00:55:30]     let mut _0: ();
[00:55:30]     scope 1 {
[00:55:30]     }
[00:55:30]     let mut _2: bool;
[00:55:30]     let mut _3: *mut i32;
[00:55:30]     let mut _4: *mut i32;
[00:55:30]     let mut _5: &ReErased mut i32;
[00:55:30]     bb0: {                              
[00:55:30]         StorageLive(_3);
[00:55:30]         StorageLive(_4);
[00:55:30]         StorageLive(_5);
[00:55:30]         Validate(Suspend(ReScope(Node(ItemLocalId(2)))), [(*_1): i32]);
[00:55:30]         _5 = &ReErased mut (*_1);
[00:55:30]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(2)))]);
[00:55:30]         _4 = move _5 as *mut i32 (Misc);
[00:55:30]         _3 = move _4;
[00:55:30]         EndRegion(ReScope(Node(ItemLocalId(2))));
[00:55:30]         StorageDead(_4);
[00:55:30]         StorageDead(_5);
[00:55:30]         Validate(Release, [_2: bool, _3: *mut i32]);
[00:55:30]         _2 = const write_42(move _3) -> bb1;
[00:55:30]     }
[00:55:30]     bb1: {                              
[00:55:30]         Validate(Acquire, [_2: bool]);
[00:55:30]         StorageDead(_3);
[00:55:30]         _0 = ();
[00:55:30]         return;
[00:55:30] }', tools/compiletest/src/runtest.rs:2815:13
[00:55:30] 
[00:55:30] 
[00:55:30] failures:
---
[00:55:30] test result: FAILED. 48 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[00:55:30] 
[00:55:30] 
[00:55:30] 
[00:55:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:30] expected suFilesystem      Size  Used Avail Use% Mounted on
tmpfs           1.5G  284K  1.5G   1% /run
/dev/sda1        30G  9.6G   19G  35% /
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
