plain
[00:45:40] .................................................................................................... 2200/4591
[00:45:43] .............i...................................................................................... 2300/4591
[00:45:47] .................................................................................................... 2400/4591
[00:45:50] .................................................................................................... 2500/4591
[00:45:53] ..........................iiiiiiiii................................................................. 2600/4591
[00:45:59] .................................................................................................... 2800/4591
[00:46:02] .................................................................................................... 2900/4591
[00:46:05] ..............................................i..................................................... 3000/4591
[00:46:07] .................................................................................................... 3100/4591
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:19] 
[00:57:19] running 47 tests
[00:57:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:57:37] .............FFF..F....................F..FF...
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/end_region_5.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/end_region_5.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/end_region_5.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn main::{{closure}}(_1: [closure@NodeId(18) d:&\'14s D]) -> i32 {"
[00:57:37] Test Name: rustc.main-{{closure}}.SimplifyCfg-qualify-consts.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn main::{{closure}}(_1: [closure@NodeId(18) d:&'14s D]) -> i32 {
[00:57:37]    let mut _0: i32;
[00:57:37]    bb0: {
[00:57:37]        _0 = ((*(_1.0: &'14s D)).0: i32);
[00:57:37]        return;
[00:57:37] Actual:
[00:57:37] Actual:
[00:57:37] closure main::{{closure}}(_1: [closure@NodeId(18) d:&'14s D]) -> i32{
[00:57:37]     let mut _0: i32;
[00:57:37]     bb0: {                              
[00:57:37]         _0 = ((*(_1.0: &'14s D)).0: i32);
[00:57:37]         return;
[00:57:37] }', tools/compiletest/src/runtest.rs:2932:13
[00:57:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/end_region_6.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/end_region_6.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/end_region_6.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn main::{{closure}}(_1: [closure@NodeId(22) d:&\'19s D]) -> i32 {"
[00:57:37] Test Name: rustc.main-{{closure}}.SimplifyCfg-qualify-consts.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn main::{{closure}}(_1: [closure@NodeId(22) d:&'19s D]) -> i32 {
[00:57:37]     let mut _0: i32;
[00:57:37] ... (elided)
[00:57:37]     let _2: &'16_0rs D;
[00:57:37] ... (elided)
[00:57:37]     bb0: {
[00:57:37]         StorageLive(_2);
[00:57:37]         _2 = &'16_0rs (*(_1.0: &'19s D));
[00:57:37]         FakeRead(ForLet, _2);
[00:57:37]         _0 = ((*_2).0: i32);
[00:57:37]         EndRegion('16_0rs);
[00:57:37]         StorageDead(_2);
[00:57:37]         return;
[00:57:37] Actual:
[00:57:37] Actual:
[00:57:37] closure main::{{closure}}(_1: [closure@NodeId(22) d:&'19s D]) -> i32{
[00:57:37]     let mut _0: i32;
[00:57:37]     scope 1 {
[00:57:37]     scope 2 {
[00:57:37]     scope 2 {
[00:57:37]         let _2: &'16_0rs D;
[00:57:37]     }
[00:57:37]     bb0: {                              
[00:57:37]         StorageLive(_2);
[00:57:37]         _2 = &'16_0rs (*(_1.0: &'19s D));
[00:57:37]         FakeRead(ForLet, _2);
[00:57:37]         _0 = ((*_2).0: i32);
[00:57:37]         EndRegion('16_0rs);
[00:57:37]         StorageDead(_2);
[00:57:37]         return;
[00:57:37] }', tools/compiletest/src/runtest.rs:2932:13
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/end_region_7.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn main::{{closure}}(_1: [closure@NodeId(22) d:D]) -> i32 {"
[00:57:37] Test Name: rustc.main-{{closure}}.SimplifyCfg-qualify-consts.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn main::{{closure}}(_1: [closure@NodeId(22) d:D]) -> i32 {
[00:57:37]     let mut _0: i32;
[00:57:37] ... (elided)
[00:57:37]     let _2: &'16_0rs D;
[00:57:37] ... (elided)
[00:57:37]     bb0: {
[00:57:37]         StorageLive(_2);
[00:57:37]         _2 = &'16_0rs (_1.0: D);
[00:57:37]         FakeRead(ForLet, _2);
[00:57:37]         _0 = ((*_2).0: i32);
[00:57:37]         EndRegion('16_0rs);
[00:57:37]         StorageDead(_2);
[00:57:37]         drop(_1) -> [return: bb2, unwind: bb1];
[00:57:37]     bb1: {
[00:57:37]         resume;
[00:57:37]     }
[00:57:37]     bb2: {
[00:57:37]     bb2: {
[00:57:37]         return;
[00:57:37]     }
[00:57:37] }
[00:57:37] Actual:
[00:57:37] closure main::{{closure}}(_1: [closure@NodeId(22) d:D]) -> i32{
[00:57:37]     let mut _0: i32;
[00:57:37]     scope 1 {
[00:57:37]     scope 2 {
[00:57:37]     scope 2 {
[00:57:37]         let _2: &'16_0rs D;
[00:57:37]     }
[00:57:37]     bb0: {                              
[00:57:37]         StorageLive(_2);
[00:57:37]         _2 = &'16_0rs (_1.0: D);
[00:57:37]         FakeRead(ForLet, _2);
[00:57:37]         _0 = ((*_2).0: i32);
[00:57:37]         EndRegion('16_0rs);
[00:57:37]         StorageDead(_2);
[00:57:37]         drop(_1) -> [return: bb2, unwind: bb1];
[00:57:37]     bb1: {
[00:57:37]         resume;
[00:57:37]     }
[00:57:37]     }
[00:57:37]     bb2: {                              
[00:57:37]         return;
[00:57:37] }', tools/compiletest/src/runtest.rs:2932:13
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/end_region_8.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/end_region_8.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/end_region_8.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn main::{{closure}}(_1: [closure@NodeId(22) r:&\'19s D]) -> i32 {"
[00:57:37] Test Name: rustc.main-{{closure}}.SimplifyCfg-qualify-consts.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn main::{{closure}}(_1: [closure@NodeId(22) r:&'19s D]) -> i32 {
[00:57:37]     let mut _0: i32;
[00:57:37]     bb0: {
[00:57:37]         _0 = ((*(_1.0: &'21_1rs D)).0: i32);
[00:57:37]         return;
[00:57:37] }
[00:57:37] Actual:
[00:57:37] Actual:
[00:57:37] closure main::{{closure}}(_1: [closure@NodeId(22) r:&'19s D]) -> i32{
[00:57:37]     let mut _0: i32;
[00:57:37]     bb0: {                              
[00:57:37]         _0 = ((*(_1.0: &'21_1rs D)).0: i32);
[00:57:37]         return;
[00:57:37] }', tools/compiletest/src/runtest.rs:2932:13
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32 {"
[00:57:37] Test Name: rustc.main-{{closure}}.EraseRegions.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32 {
[00:57:37] ... (elided)
[00:57:37]     bb0: {
[00:57:37]         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[00:57:37]         StorageLive(_3);
[00:57:37]         Validate(Suspend(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0 })), [(*_2): i32]);
[00:57:37]         _3 = &ReErased (*_2);
[00:57:37]         Validate(Acquire, [(*_3): i32/ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0 }) (imm)]);
[00:57:37]         _0 = (*_3);
[00:57:37]         EndRegion(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0 }));
[00:57:37]         StorageDead(_3);
[00:57:37]         return;
[00:57:37] }
[00:57:37] Actual:
[00:57:37] Actual:
[00:57:37] closure main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32{
[00:57:37]     let mut _0: i32;
[00:57:37]     scope 1 {
[00:57:37]     scope 2 {
[00:57:37]     scope 2 {
[00:57:37]         let _3: &ReErased i32;
[00:57:37]     }
[00:57:37]     bb0: {                              
[00:57:37]         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[00:57:37]         StorageLive(_3);
[00:57:37]         Validate(Suspend(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0})), [(*_2): i32]);
[00:57:37]         _3 = &ReErased (*_2);
[00:57:37]         Validate(Acquire, [(*_3): i32/ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0}) (imm)]);
[00:57:37]         _0 = (*_3);
[00:57:37]         EndRegion(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0}));
[00:57:37]         StorageDead(_3);
[00:57:37]         return;
[00:57:37] }', tools/compiletest/src/runtest.rs:2932:13
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/validate_4.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/validate_4.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/validate_4.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn write_42::{{closure}}(_1: &ReErased [closure@NodeId(22)], _2: *mut i32) -> () {"
[00:57:37] Test Name: rustc.write_42-{{closure}}.EraseRegions.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn write_42::{{closure}}(_1: &ReErased [closure@NodeId(22)], _2: *mut i32) -> () {
[00:57:37] ... (elided)
[00:57:37]     bb0: {
[00:57:37]         Validate(Acquire, [_1: &ReFree(DefId(0/1:9 ~ validate_4[317d]::write_42[0]::{{closure}}[0]), BrEnv) [closure@NodeId(22)], _2: *mut i32]);
[00:57:37]         Validate(Release, [_1: &ReFree(DefId(0/1:9 ~ validate_4[317d]::write_42[0]::{{closure}}[0]), BrEnv) [closure@NodeId(22)], _2: *mut i32]);
[00:57:37]         (*_2) = const 23i32;
[00:57:37]         _0 = ();
[00:57:37]         return;
[00:57:37] }
[00:57:37] Actual:
[00:57:37] Actual:
[00:57:37] closure write_42::{{closure}}(_1: &ReErased [closure@NodeId(22)], _2: *mut i32) -> (){
[00:57:37]     let mut _0: ();
[00:57:37]     bb0: {                              
[00:57:37]         Validate(Acquire, [_1: &ReFree(DefId(0/1:9 ~ validate_4[317d]::write_42[0]::{{closure}}[0]), BrEnv) [closure@NodeId(22)], _2: *mut i32]);
[00:57:37]         Validate(Release, [_1: &ReFree(DefId(0/1:9 ~ validate_4[317d]::write_42[0]::{{closure}}[0]), BrEnv) [closure@NodeId(22)], _2: *mut i32]);
[00:57:37]         (*_2) = const 23i32;
[00:57:37]         _0 = ();
[00:57:37]         return;
[00:57:37] }', tools/compiletest/src/runtest.rs:2932:13
[00:57:37] 
[00:57:37] ---- [mir-opt] mir-opt/validate_5.rs stdout ----
[00:57:37] ---- [mir-opt] mir-opt/validate_5.rs stdout ----
[00:57:37] thread '[mir-opt] mir-opt/validate_5.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:37] Expected Line: "fn main::{{closure}}(_1: &ReErased [closure@NodeId(46)], _2: &ReErased mut i32) -> bool {"
[00:57:37] Test Name: rustc.main-{{closure}}.EraseRegions.after.mir
[00:57:37] ... (elided)
[00:57:37] ... (elided)
[00:57:37] fn main::{{closure}}(_1: &ReErased [closure@NodeId(46)], _2: &ReErased mut i32) -> bool {
[00:57:37] ... (elided)
[00:57:37]     bb0: {
[00:57:37]         Validate(Acquire, [_1: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(46)], _2: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[00:57:37]         StorageLive(_3);
[00:57:37]         StorageLive(_4);
[00:57:37]         StorageLive(_5);
[00:57:37]         Validate(Suspend(ReScope(Node(ItemLocalId(12)))), [(*_2): i32]);
[00:57:37]         _5 = &ReErased mut (*_2);
[00:57:37]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(12)))]);
[00:57:37]         _4 = move _5 as *mut i32 (Misc);
[00:57:37]         _3 = move _4;
[00:57:37]         EndRegion(ReScope(Node(ItemLocalId(12))));
[00:57:37]         StorageDead(_4);
[00:57:37]         StorageDead(_5);
[00:57:37]         Validate(Release, [_0: bool, _3: *mut i32]);
[00:57:37]         _0 = const write_42(move _3) -> bb1;
[00:57:37] ... (elided)
[00:57:37] }
[00:57:37] Actual:
[00:57:37] Actual:
[00:57:37] closure main::{{closure}}(_1: &ReErased [closure@NodeId(46)], _2: &ReErased mut i32) -> bool{
[00:57:37]     let mut _0: bool;
[00:57:37]     let mut _3: *mut i32;
[00:57:37]     let mut _4: *mut i32;
[00:57:37]     let mut _5: &ReErased mut i32;
[00:57:37]     bb0: {                              
[00:57:37]         Validate(Acquire, [_1: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(46)], _2: &ReFree(DefId(0/1:9 ~ validate_5[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[00:57:37]         StorageLive(_3);
[00:57:37]         StorageLive(_4);
[00:57:37]         StorageLive(_5);
[00:57:37]         Validate(Suspend(ReScope(Node(ItemLocalId(12)))), [(*_2): i32]);
[00:57:37]         _5 = &ReErased mut (*_2);
[00:57:37]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(12)))]);
[00:57:37]         _4 = move _5 as *mut i32 (Misc);inux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:37] 
[00:57:37] 
[00:57:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:37] Build completed unsuccessfully in 0:16:10
[00:57:37] Build completed unsuccessfully in 0:16:10
[00:57:37] Makefile:58: recipe for target 'check' failed
[00:57:37] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02bad4c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01753ca9:start=1539359108129548980,finish=1539359108133535706,duration=3986726
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16d58bf6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033
