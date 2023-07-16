plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..................................i................................
failures:

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
1 - // MIR for `BAR` before PromoteTemps
2 + // MIR for `BAR` after PromoteTemps
+   
+   
4   static mut BAR: *const &i32 = {
5       let mut _0: *const &i32;             // return place in scope 0 at $DIR/const-promotion-extern-static.rs:9:17: 9:28
6       let mut _1: &[&i32];                 // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44

9       let mut _4: &i32;                    // in scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
10       let _5: &i32;                        // in scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
11 +     let mut _6: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+   
13       bb0: {
13       bb0: {
14           StorageLive(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
15           StorageLive(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44

35                                            // + span: $DIR/const-promotion-extern-static.rs:9:36: 9:42
36                                            // + literal: Const { ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}, val: Value(Scalar(<ZST>)) }
- 
+   
39       bb1: {
39       bb1: {
40 -         StorageDead(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
41 -         StorageDead(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44

42           StorageDead(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:43: 9:44
43           return;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:45
- 
+   
+   
46       bb2 (cleanup): {
47           resume;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:45

49 - }
- -
+ - 
+ - 
51 - alloc1 (static: Y, size: 4, align: 4) {
52 -     2a 00 00 00                                     │ *...

- 
+   
55 
55 

thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3386:25


failures:
    [mir-opt] mir-opt/const-promotion-extern-static.rs
