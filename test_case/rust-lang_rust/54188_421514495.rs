plain
[00:54:03] ....................................................................................................
[00:54:06] ....................................................i...............................................
[00:54:09] ....................................................................................................
[00:54:12] ....................................................................................................
[00:54:15] iiiiiiiii...........................................................................................
[00:54:21] ....................................................................................................
[00:54:24] .................................................................................i..................
[00:54:27] ....................................................................................................
[00:54:30] ..................................i.i..ii...........................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:27] 
[01:01:27] running 45 tests
[01:01:31] ERROR 2018-09-15T00:08:51Z: compiletest::runtest: None
[01:01:31] ERROR 2018-09-15T00:08:51Z: compiletest::runtest: None
[01:01:36] ERROR 2018-09-15T00:08:56Z: compiletest::runtest: Some(" bb3: {")
 main::{{closure}}(_1: [closure@NodeId(22) d:&'19s D]) -> i32{
[01:01:46]     let mut _0: i32;
[01:01:46]     scope 1 {
[01:01:46]     scope 2 {
[01:01:46]     scope 2 {
[01:01:46]         let _2: &'16_0rs D;
[01:01:46]     }
[01:01:46]     bb0: {                              
[01:01:46]         StorageLive(_2);
[01:01:46]         _2 = &'16_0rs (*(_1.0: &'19s D));
[01:01:46]         FakeRead(ForLet, _2);
[01:01:46]         _0 = ((*_2).0: i32);
[01:01:46]         EndRegion('16_0rs);
[01:01:46]         StorageDead(_2);
[01:01:46]         return;
[01:01:46] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:46] 
[01:01:46] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[01:01:46] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[01:01:46] thread '[mir-opt] mir-opt/end_region_7.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:46] Current block: None
[01:01:46] Actual Line: "        FakeRead(ForLet, _2);"
[01:01:46] Expected Line: "        _0 = ((*_2).0: i32);"
[01:01:46] Test Name: rustc.main-{{closure}}.SimplifyCfg-qualify-consts.after.mir
[01:01:46] ... (elided)
[01:01:46] ... (elided)
[01:01:46] fn main::{{closure}}(_1: [closure@NodeId(22) d:D]) -> i32 {
[01:01:46]     let mut _0: i32;
[01:01:46] ... (elided)
[01:01:46]     let _2: &'16_0rs D;
[01:01:46] ... (elided)
[01:01:46]     bb0: {
[01:01:46]         StorageLive(_2);
[01:01:46]         _2 = &'16_0rs (_1.0: D);
[01:01:46]         _0 = ((*_2).0: i32);
[01:01:46]         EndRegion('16_0rs);
[01:01:46]         StorageDead(_2);
[01:01:46]         drop(_1) -> [return: bb2, unwind: bb1];
[01:01:46]01:01:46]         StorageLive(_8);
[01:01:46]         _8 = ((_2 as Some).0: i32);
[01:01:46]         StorageLive(_12);
[01:01:46]         _12 = _8;
[01:01:46]         _1 = (const 2i32, move _12);
[01:01:46]         StorageDead(_12);
[01:01:46]         goto -> bb13;
[01:01:46]     }
[01:01:46]     bb13: {                             
[01:01:46]         StorageDead(_8);
[01:01:46]         StorageDead(_5);
[01:01:46]         StorageDead(_10);
[01:01:46]         StorageDead(_7);
[01:01:46]         StorageDead(_1);
[01:01:46]         StorageDead(_2);
[01:01:46]         _0 = ();
[01:01:46]         return;
[01:01:46] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:46] 
[01:01:46] 
[01:01:46] failures:
---
[01:01:46] test result: FAILED. 42 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:01:46] 
[01:01:46] 
[01:01:46] 
[01:01:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14da78c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
