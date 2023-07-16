plain
failures:

---- [mir-opt] src/test/mir-opt/inline/asm-unwind.rs stdout ----
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/asm-unwind.rs:+0:15: +0:15
6       let _1: ();                          // in scope 0 at $DIR/asm-unwind.rs:+1:5: +1:10
- +     scope 1 (inlined foo) {              // at $DIR/asm-unwind.rs:20:5: 20:10
- +         let _2: D;                       // in scope 1 at $DIR/asm-unwind.rs:14:9: 14:11
+ +     scope 1 (inlined foo) {              // at $DIR/asm-unwind.rs:21:5: 21:10
+ +         let _2: D;                       // in scope 1 at $DIR/asm-unwind.rs:15:9: 15:11
9 +         scope 2 {
- +             debug _d => _2;              // in scope 2 at $DIR/asm-unwind.rs:14:9: 14:11
+ +             debug _d => _2;              // in scope 2 at $DIR/asm-unwind.rs:15:9: 15:11
11 +             scope 3 {
13 +         }


17           StorageLive(_1);                 // scope 0 at $DIR/asm-unwind.rs:+1:5: +1:10
18 -         _1 = foo() -> bb1;               // scope 0 at $DIR/asm-unwind.rs:+1:5: +1:10
19 -                                          // mir::Constant
- -                                          // + span: $DIR/asm-unwind.rs:20:5: 20:8
+ -                                          // + span: $DIR/asm-unwind.rs:21:5: 21:8
21 -                                          // + literal: Const { ty: fn() {foo}, val: Value(<ZST>) }
- +         StorageLive(_2);                 // scope 1 at $DIR/asm-unwind.rs:14:9: 14:11
- +         asm!("", options(MAY_UNWIND)) -> [return: bb1, unwind: bb3]; // scope 3 at $DIR/asm-unwind.rs:15:14: 15:54
+ +         StorageLive(_2);                 // scope 1 at $DIR/asm-unwind.rs:15:9: 15:11
+ +         asm!("", options(MAY_UNWIND)) -> [return: bb1, unwind: bb3]; // scope 3 at $DIR/asm-unwind.rs:16:14: 16:54
25   
26       bb1: {


- +         drop(_2) -> bb2;                 // scope 1 at $DIR/asm-unwind.rs:16:1: 16:2
+ +         drop(_2) -> bb2;                 // scope 1 at $DIR/asm-unwind.rs:17:1: 17:2
29 + 
30 +     bb2: {


- +         StorageDead(_2);                 // scope 1 at $DIR/asm-unwind.rs:16:1: 16:2
+ +         StorageDead(_2);                 // scope 1 at $DIR/asm-unwind.rs:17:1: 17:2
32           StorageDead(_1);                 // scope 0 at $DIR/asm-unwind.rs:+1:10: +1:11
33           _0 = const ();                   // scope 0 at $DIR/asm-unwind.rs:+0:15: +2:2
34           return;                          // scope 0 at $DIR/asm-unwind.rs:+2:2: +2:2
35 +     }
36 + 
36 + 
37 +     bb3 (cleanup): {
- +         drop(_2) -> bb4;                 // scope 1 at $DIR/asm-unwind.rs:16:1: 16:2
+ +         drop(_2) -> bb4;                 // scope 1 at $DIR/asm-unwind.rs:17:1: 17:2
40 + 
40 + 
41 +     bb4 (cleanup): {

- +         resume;                          // scope 1 at $DIR/asm-unwind.rs:13:1: 16:2
+ +         resume;                          // scope 1 at $DIR/asm-unwind.rs:14:1: 17:2
44   }
45   


thread '[mir-opt] src/test/mir-opt/inline/asm-unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/asm_unwind.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3515:25


failures:
    [mir-opt] src/test/mir-opt/inline/asm-unwind.rs
