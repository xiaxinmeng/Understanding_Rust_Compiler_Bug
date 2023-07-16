plain
 finished in 0.593 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 178 tests
i.......F..............................i................................................ 88/178
......i...............................F..................i........F....................F 176/178
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs stdout ----
40 -         StorageDead(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
41 -         StorageDead(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
42           StorageDead(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
-           return;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:45
+           return;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:15
45   
45   
46       bb2 (cleanup): {

-           resume;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:45
+           resume;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:15
49 - }
50 - 


thread '[mir-opt] src/test/mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3437:25

---- [mir-opt] src/test/mir-opt/issue-41697.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-41697.rs stdout ----
- // MIR for `<impl at $DIR/issue-41697.rs:18:1: 22:2>::{constant#0}` after SimplifyCfg-promote-consts
+ // MIR for `<impl at $DIR/issue-41697.rs:18:1: 18:23>::{constant#0}` after SimplifyCfg-promote-consts
2 
- <impl at $DIR/issue-41697.rs:18:1: 22:2>::{constant#0}: usize = {
+ <impl at $DIR/issue-41697.rs:18:1: 18:23>::{constant#0}: usize = {
4     let mut _0: usize;                   // return place in scope 0 at $DIR/issue-41697.rs:18:19: 18:22
5     let mut _1: (usize, bool);           // in scope 0 at $DIR/issue-41697.rs:18:19: 18:22


thread '[mir-opt] src/test/mir-opt/issue-41697.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_41697.{impl#0}-{constant#0}.SimplifyCfg-promote-consts.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
- // MIR for `<impl at $DIR/retag.rs:11:1: 19:2>::foo` after SimplifyCfg-elaborate-drops
+ // MIR for `<impl at $DIR/retag.rs:11:1: 11:10>::foo` after SimplifyCfg-elaborate-drops
2 
- fn <impl at $DIR/retag.rs:11:1: 19:2>::foo(_1: &Test, _2: &mut i32) -> &mut i32 {
+ fn <impl at $DIR/retag.rs:11:1: 11:10>::foo(_1: &Test, _2: &mut i32) -> &mut i32 {
4     debug self => _1;                    // in scope 0 at $DIR/retag.rs:13:16: 13:21
5     debug x => _2;                       // in scope 0 at $DIR/retag.rs:13:23: 13:24
6     let mut _0: &mut i32;                // return place in scope 0 at $DIR/retag.rs:13:42: 13:53

thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.{impl#0}-foo.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
- // MIR for `<impl at $DIR/unusual-item-types.rs:9:1: 11:2>::ASSOCIATED_CONSTANT` 0 mir_map
+ // MIR for `<impl at $DIR/unusual-item-types.rs:9:1: 9:7>::ASSOCIATED_CONSTANT` 0 mir_map
2 
- const <impl at $DIR/unusual-item-types.rs:9:1: 11:2>::ASSOCIATED_CONSTANT: i32 = {
+ const <impl at $DIR/unusual-item-types.rs:9:1: 9:7>::ASSOCIATED_CONSTANT: i32 = {
4     let mut _0: i32;                     // return place in scope 0 at $DIR/unusual-item-types.rs:10:32: 10:35
6     bb0: {


7         _0 = const 2_i32;                // scope 0 at $DIR/unusual-item-types.rs:10:38: 10:39
-         return;                          // scope 0 at $DIR/unusual-item-types.rs:10:5: 10:40
+         return;                          // scope 0 at $DIR/unusual-item-types.rs:10:5: 10:30
10 }
11 


thread '[mir-opt] src/test/mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.{impl#0}-ASSOCIATED_CONSTANT.mir_map.0.64bit.mir', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs stdout ----
---- [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs stdout ----
198         _0 = &(*_1);                     // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
199         StorageDead(_5);                 // scope 0 at $DIR/storage_live_dead_in_statics.rs:23:1: 23:2
200         StorageDead(_1);                 // scope 0 at $DIR/storage_live_dead_in_statics.rs:23:1: 23:2
-         return;                          // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:1: 23:3
+         return;                          // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:1: 5:11
203 }
204 


thread '[mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/storage_live_dead_in_statics.XXX.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3437:25

failures:
    [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs
    [mir-opt] src/test/mir-opt/issue-41697.rs
