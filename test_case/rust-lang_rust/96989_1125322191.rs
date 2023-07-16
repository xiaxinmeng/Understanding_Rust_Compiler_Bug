plain
 finished in 0.627 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 174 tests
......................................i...........................................F.F... 88/174
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/inline/cycle.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/cycle.rs stdout ----
4   fn g() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/cycle.rs:11:8: 11:8
6       let _1: ();                          // in scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +     let mut _2: fn() {main};             // in scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +     let mut _5: ();                      // in scope 0 at $DIR/cycle.rs:6:5: 6:8
+ +     scope 1 (inlined f::<fn() {main}>) { // at $DIR/cycle.rs:12:5: 12:12
+ +         debug g => _2;                   // in scope 1 at $DIR/cycle.rs:5:6: 5:7
+ +         let _3: ();                      // in scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         let mut _4: &fn() {main};        // in scope 1 at $DIR/cycle.rs:6:5: 6:6
+ +         scope 2 (inlined <fn() {main} as Fn<()>>::call - shim(fn() {main})) { // at $DIR/cycle.rs:6:5: 6:8
+ +         }
+ +     }
8       bb0: {
8       bb0: {
9           StorageLive(_1);                 // scope 0 at $DIR/cycle.rs:12:5: 12:12

-           _1 = f::<fn() {main}>(main) -> bb1; // scope 0 at $DIR/cycle.rs:12:5: 12:12
+ -         _1 = f::<fn() {main}>(main) -> bb1; // scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +         StorageLive(_2);                 // scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +         _2 = main;                       // scope 0 at $DIR/cycle.rs:12:5: 12:12
11                                            // mir::Constant
-                                            // + span: $DIR/cycle.rs:12:5: 12:6
-                                            // + literal: Const { ty: fn(fn() {main}) {f::<fn() {main}>}, val: Value(Scalar(<ZST>)) }
-                                            // mir::Constant
+ -                                          // + span: $DIR/cycle.rs:12:5: 12:6
+ -                                          // + literal: Const { ty: fn(fn() {main}) {f::<fn() {main}>}, val: Value(Scalar(<ZST>)) }
+ -                                          // mir::Constant
15                                            // + span: $DIR/cycle.rs:12:7: 12:11
16                                            // + literal: Const { ty: fn() {main}, val: Value(Scalar(<ZST>)) }
+ +         StorageLive(_3);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         StorageLive(_4);                 // scope 1 at $DIR/cycle.rs:6:5: 6:6
+ +         _4 = &_2;                        // scope 1 at $DIR/cycle.rs:6:5: 6:6
+ +         StorageLive(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         _5 = const ();                   // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         _3 = move (*_4)() -> [return: bb4, unwind: bb2]; // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
18   
19       bb1: {


+ +         StorageDead(_2);                 // scope 0 at $DIR/cycle.rs:12:5: 12:12
20           StorageDead(_1);                 // scope 0 at $DIR/cycle.rs:12:12: 12:13
21           _0 = const ();                   // scope 0 at $DIR/cycle.rs:11:8: 13:2
22           return;                          // scope 0 at $DIR/cycle.rs:13:2: 13:2
+ +     }
+ + 
+ + 
+ +     bb2 (cleanup): {
+ +         drop(_2) -> bb3;                 // scope 1 at $DIR/cycle.rs:7:1: 7:2
+ +     }
+ + 
+ +     bb3 (cleanup): {
+ +         resume;                          // scope 1 at $DIR/cycle.rs:5:1: 7:2
+ +     }
+ +     bb4: {
+ +     bb4: {
+ +         StorageDead(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         StorageDead(_4);                 // scope 1 at $DIR/cycle.rs:6:7: 6:8
+ +         StorageDead(_3);                 // scope 1 at $DIR/cycle.rs:6:8: 6:9
+ +         drop(_2) -> bb1;                 // scope 1 at $DIR/cycle.rs:7:1: 7:2
24   }
25   


thread '[mir-opt] src/test/mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/cycle.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25

---- [mir-opt] src/test/mir-opt/inline/inline-cycle-generic.rs stdout ----
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-cycle-generic.rs:8:11: 8:11
6       let _1: ();                          // in scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +     scope 1 (inlined <C as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +         scope 2 (inlined <B<A> as Call>::call) { // at $DIR/inline-cycle-generic.rs:38:9: 38:31
- +         }
- +     }
12       bb0: {
12       bb0: {
13           StorageLive(_1);                 // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24

- -         _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +         _1 = <A as Call>::call() -> bb1; // scope 2 at $DIR/inline-cycle-generic.rs:31:9: 31:28
+           _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
16                                            // mir::Constant
- -                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:22
- -                                          // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
- +                                          // + span: $DIR/inline-cycle-generic.rs:31:9: 31:26
- +                                          // + literal: Const { ty: fn() {<A as Call>::call}, val: Value(Scalar(<ZST>)) }
+                                            // + span: $DIR/inline-cycle-generic.rs:9:5: 9:22
+                                            // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
22   
23       bb1: {


thread '[mir-opt] src/test/mir-opt/inline/inline-cycle-generic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_cycle_generic.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25

failures:
    [mir-opt] src/test/mir-opt/inline/cycle.rs
    [mir-opt] src/test/mir-opt/inline/inline-cycle-generic.rs
