plain
[00:53:40] ....................................................................................................
[00:53:43] ....................................................i...............................................
[00:53:45] ....................................................................................................
[00:53:48] ....................................................................................................
[00:53:51] iiiiiiiii...........................................................................................
[00:53:57] ....................................................................................................
[00:54:00] .................................................................................i..................
[00:54:03] ....................................................................................................
[00:54:06] ..................................i.i..ii...........................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:39] 
[01:00:39] running 46 tests
[01:00:52] ERROR 2018-09-11T11:09:48Z: compiletest::runtest: Some("    bb0: {")
[01:00:54] ERROR 2018-09-11T11:09:50Z: compiletest::runtest: Some("        Validate(Acquire, [_1: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(46)], _2: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);")
[01:00:57] ......................................F.F.F...
[01:00:57] 
[01:00:57] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:00:57] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:00:57] thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:00:57] Expected Line: "        Validate(Suspend(ReScope(Node(ItemLocalId(10)))), [_1: i32]);"
[01:00:57] Test Name: rustc.main.EraseRegions.after.mir
[01:00:57] ... (elided)
[01:00:57] fn main() -> () {
[01:00:57] ... (elided)
[01:00:57]     bb0: {
[01:00:57]     bb0: {
[01:00:57] ... (elided)
[01:00:57]         Validate(Suspend(ReScope(Node(ItemLocalId(10)))), [_1: i32]);
[01:00:57]         _6 = &ReErased mut _1;
[01:00:57]         Validate(Acquire, [(*_6): i32/ReScope(Node(ItemLocalId(10)))]);
[01:00:57]         Validate(Suspend(ReScope(Node(ItemLocalId(10)))), [(*_6): i32/ReScope(Node(ItemLocalId(10)))]);
[01:00:57]         _5 = &ReErased mut (*_6);
[01:00:57]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(10)))]);
[01:00:57]         Validate(Release, [_2: (), _3: &ReScope(Node(ItemLocalId(10))) Test, _5: &ReScope(Node(ItemLocalId(10))) mut i32]);
[01:00:57]         _2 = const Test::foo(move _3, move _5) -> bb1;
[01:00:57]     bb1: {
[01:00:57]     bb1: {
[01:00:57]         Validate(Acquire, [_2: ()]);
[01:00:57]         EndRegion(ReScope(Node(ItemLocalId(10))));
[01:00:57] ... (elided)
[01:00:57]         return;
[01:00:57] }
[01:00:57] Actual:
[01:00:57] fn main() -> (){
[01:00:57] fn main() -> (){
[01:00:57]     let mut _0: ();
[01:00:57]     scope 1 {
[01:00:57]         scope 3 {
[01:00:57]         scope 4 {
[01:00:57]         scope 4 {
[01:00:57]             let _7: [closure@NodeId(50)];
[01:00:57]     }
[01:00:57]     scope 2 {
[01:00:57]         let mut _1: i32;
[01:00:57]     }
[01:00:57]     }
[01:00:57]     let mut _2: ();
[01:00:57]     let mut _3: &ReErased Test;
[01:00:57]     let _4: Test;
[01:00:57]     let mut _5: &ReErased mut i32;
[01:00:57]     let mut _6: &ReErased mut i32;
[01:00:57]     let mut _8: i32;
[01:00:57]     let mut _9: &ReErased [closure@NodeId(50)];
[01:00:57]     let mut _10: (&ReErased mut i32,);
[01:00:57]     let mut _11: &ReErased mut i32;
[01:00:57]     let mut _12: &ReErased mut i32;
[01:00:57]     bb0: {                              
[01:00:57]         StorageLive(_1);
[01:00:57]         _1 = const 0i32;
[01:00:57]         StorageLive(_3);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(10), data: Node })), [(promoted[0]: Test): Test (imm)]);
[01:00:57]         _3 = &ReErased (promoted[0]: Test);
[01:00:57]         Validate(Acquire, [(*_3): Test/ReScope(Scope { id: ItemLocalId(10), data: Node }) (imm)]);
[01:00:57]         StorageLive(_5);
[01:00:57]         StorageLive(_6);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(10), data: Node })), [_1: i32]);
[01:00:57]         _6 = &ReErased mut _1;
[01:00:57]         Validate(Acquire, [(*_6): i32/ReScope(Scope { id: ItemLocalId(10), data: Node })]);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(10), data: Node })), [(*_6): i32/ReScope(Scope { id: ItemLocalId(10), data: Node })]);
[01:00:57]         _5 = &ReErased mut (*_6);
[01:00:57]         Validate(Acquire, [(*_5): i32/ReScope(Scope { id: ItemLocalId(10), data: Node })]);
[01:00:57]         Validate(Release, [_2: (), _3: &ReScope(Scope { id: ItemLocalId(10), data: Node }) Test, _5: &ReScope(Scope { id: ItemLocalId(10), data: Node }) mut i32]);
[01:00:57]         _2 = const Test::foo(move _3, move _5) -> bb1;
[01:00:57]     }
[01:00:57]     bb1: {                              
[01:00:57]         Validate(Acquire, [_2: ()]);
[01:00:57]         EndRegion(ReScope(Scope { id: ItemLocalId(10), data: Node }));
[01:00:57]         StorageDead(_5);
[01:00:57]         StorageDead(_3);
[01:00:57]         StorageDead(_6);
[01:00:57]         StorageLive(_7);
[01:00:57]         _7 = [closure@NodeId(50)];
[01:00:57]         StorageLive(_9);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(34), data: Node })), [_7: [closure@NodeId(50)]]);
[01:00:57]         _9 = &ReErased _7;
[01:00:57]         Validate(Acquire, [(*_9): [closure@NodeId(50)]/ReScope(Scope { id: ItemLocalId(34), data: Node }) (imm)]);
[01:00:57]         StorageLive(_10);
[01:00:57]         StorageLive(_11);
[01:00:57]         StorageLive(_12);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(34), data: Node })), [_1: i32]);
[01:00:57]         _12 = &ReErased mut _1;
[01:00:57]         Validate(Acquire, [(*_12): i32/ReScope(Scope { id: ItemLocalId(34), data: Node })]);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(34), data: Node })), [(*_12): i32/ReScope(Scope { id: ItemLocalId(34), data: Node })]);
[01:00:57]         _11 = &ReErased mut (*_12);
[01:00:57]         Validate(Acquire, [(*_11): i32/ReScope(Scope { id: ItemLocalId(34), data: Node })]);
[01:00:57]         _10 = (move _11,);
[01:00:57]         Validate(Release, [_8: i32, _9: &ReScope(Scope { id: ItemLocalId(34), data: Node }) [closure@NodeId(50)], _10: (&ReScope(Scope { id: ItemLocalId(34), data: Node }) mut i32,)]);
[01:00:57]         _8 = const std::ops::Fn::call(move _9, move _10) -> bb2;
[01:00:57]     }
[01:00:57]     bb2: {                              
[01:00:57]         Validate(Acquire, [_8: i32]);
[01:00:57]         EndRegion(ReScope(Scope { id: ItemLocalId(34), data: Node }));
[01:00:57]         StorageDead(_10);
[01:00:57]         StorageDead(_11);
[01:00:57]         StorageDead(_9);
[01:00:57]         StorageDead(_12);
[01:00:57]         _0 = ();
[01:00:57]         StorageDead(_7);
[01:00:57]         StorageDead(_1);
[01:00:57]         return;
[01:00:57] }', tools/compiletest/src/runtest.rs:2851:13
[01:00:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:57] 
[01:00:57] ---- [mir-opt] mir-opt/validate_3.rs stdout ----
[01:00:57] ---- [mir-opt] mir-opt/validate_3.rs stdout ----
[01:00:57] thread '[mir-opt] mir-opt/validate_3.rs' panicked at 'Did notm)]);
[01:00:57]         _5 = &ReErased ((*_2).0: i32);
[01:00:57]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(18))) (imm)]);
[01:00:57]         Validate(Suspend(ReScope(Node(ItemLocalId(18)))), [(*_5): i32/ReScope(Node(ItemLocalId(18))) (imm)]);
[01:00:57]         _4 = &ReErased (*_5);
[01:00:57]         Validate(Acquire, [(*_4): i32/ReScope(Node(ItemLocalId(18))) (imm)]);
[01:00:57]         Validate(Release, [_3: (), _4: &ReScope(Node(ItemLocalId(18))) i32]);
[01:00:57]         _3 = const foo(move _4) -> bb1;
[01:00:57]     bb1: {
[01:00:57]     bb1: {
[01:00:57]         Validate(Acquire, [_3: ()]);
[01:00:57]         EndRegion(ReScope(Node(ItemLocalId(18))));
[01:00:57]         StorageDead(_4);
[01:00:57]         StorageDead(_5);
[01:00:57]         _0 = ();
[01:00:57]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(20), first_statement_index: 3 })));
[01:00:57]         StorageDead(_2);
[01:00:57]         StorageDead(_1);
[01:00:57]         return;
[01:00:57] }
[01:00:57] Actual:
[01:00:57] fn main() -> (){
[01:00:57] fn main() -> (){
[01:00:57]     let mut _0: ();
[01:00:57]     scope 1 {
[01:00:57]         scope 3 {
[01:00:57]         scope 4 {
[01:00:57]         scope 4 {
[01:00:57]             let _2: &ReErased Test;
[01:00:57]     }
[01:00:57]     scope 2 {
[01:00:57]         let _1: Test;
[01:00:57]     }
[01:00:57]     }
[01:00:57]     let mut _3: ();
[01:00:57]     let mut _4: &ReErased i32;
[01:00:57]     let mut _5: &ReErased i32;
[01:00:57]     bb0: {                              
[01:00:57]         StorageLive(_1);
[01:00:57]         _1 = Test { x: co32 (Misc);
[01:00:57]         _3 = move _4;
[01:00:57]         EndRegion(ReScope(Node(ItemLocalId(12))));
[01:00:57]         StorageDead(_4);
[01:00:57]         StorageDead(_5);
[01:00:57]         Validate(Release, [_0: bool, _3: *mut i32]);
[01:00:57]         _0 = const write_42(move _3) -> bb1;
[01:00:57] ... (elided)
[01:00:57] }
[01:00:57] Actual:
[01:00:57] Actual:
[01:00:57] fn main::{{closure}}(_1: &ReErased [closure@NodeId(46)], _2: &ReErased mut i32) -> bool{
[01:00:57]     let mut _0: bool;
[01:00:57]     let mut _3: *mut i32;
[01:00:57]     let mut _4: *mut i32;
[01:00:57]     let mut _5: &ReErased mut i32;
[01:00:57]     bb0: {                              
[01:00:57]         Validate(Acquire, [_1: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(46)], _2: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:00:57] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:00:57]         StorageLive(_3);
[01:00:57]         StorageLive(_4);
[01:00:57]         StorageLive(_5);
[01:00:57]         Validate(Suspend(ReScope(Scope { id: ItemLocalId(12), data: Node })), [(*_2): i32]);
[01:00:57]         _5 = &ReErased mut (*_2);
[01:00:57]         Validate(Acquire, [(*_5): i32/ReScope(Scope { id: ItemLocalId(12), data: Node })]);
[01:00:57]         _4 = move _5 as *mut i32 (Misc);
[01:00:57]         _3 = move _4;
[01:00:57]         EndRegion(ReScope(Scope { id: ItemLocalId(12), data: Node }));
[01:00:57]         StorageDead(_4);
[01:00:57]         StorageDead(_5);
[01:00:57]         Validate(Release, [_0: bool, _3: *mut i32]);
[01:00:57]         _0 = const write_42(move _3) -> bb1;
[01:00:57]     }
[01:00:57]     bb1: {                              
[01:00:57]         Validate(Acquire, [_0: bool]);
[01:00:57]         StorageDead(_3);
[01:00:57]         return;
[01:00:57] }', tools/compiletest/src/runtest.rs:2851:13
[01:00:57] 
[01:00:57] 
[01:00:57] failures:
---
[01:00:57] test result: FAILED. 43 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:00:57] 
[01:00:57] 
[01:00:57] 
[01:00:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:57] 
[01:00:57] 
[01:00:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:57] Build completed unsuccessfully in 0:15:49
[01:00:57] Build completed unsuccessfully in 0:15:49
[01:00:57] Makefile:58: recipe for target 'check' failed
[01:00:57] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02d1259c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
