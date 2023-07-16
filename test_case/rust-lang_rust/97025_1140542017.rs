plain
test [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/derefer_inline_test.rs stdout ----
25       bb1: {
26           StorageLive(_5);                 // scope 0 at $DIR/derefer_inline_test.rs:10:5: 10:12
27           _5 = ShallowInitBox(move _4, std::boxed::Box<u32>); // scope 0 at $DIR/derefer_inline_test.rs:10:5: 10:12
-           (*_5) = f() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/derefer_inline_test.rs:10:9: 10:12
+           (*_5) = f() -> bb2;              // scope 0 at $DIR/derefer_inline_test.rs:10:9: 10:12
29                                            // mir::Constant
30                                            // + span: $DIR/derefer_inline_test.rs:10:9: 10:10
31                                            // + literal: Const { ty: fn() -> Box<u32> {f}, val: Value(Scalar(<ZST>)) }
38   
39       bb3: {
39       bb3: {
40           StorageDead(_5);                 // scope 0 at $DIR/derefer_inline_test.rs:10:11: 10:12
-           drop(_1) -> [return: bb4, unwind: bb6]; // scope 0 at $DIR/derefer_inline_test.rs:10:12: 10:13
+           drop(_1) -> bb4;                 // scope 0 at $DIR/derefer_inline_test.rs:10:12: 10:13
43   
44       bb4: {

48       }
48       }
49   
50       bb5 (cleanup): {
-           goto -> bb8;                     // scope 0 at $DIR/derefer_inline_test.rs:10:11: 10:12
-   
-   
-       bb6 (cleanup): {
55           resume;                          // scope 0 at $DIR/derefer_inline_test.rs:9:1: 11:2
-   
-   
-       bb7 (cleanup): {
-           _6 = alloc::alloc::box_free::<Box<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::boxed::Box<u32>>), move (_5.1: std::alloc::Global)) -> bb6; // scope 0 at $DIR/derefer_inline_test.rs:10:11: 10:12
-                                            // mir::Constant
-                                            // + span: $DIR/derefer_inline_test.rs:10:11: 10:12
-                                            // + literal: Const { ty: unsafe fn(Unique<Box<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Box<u32>, std::alloc::Global>}, val: Value(Scalar(<ZST>)) }
-   
-   
-       bb8 (cleanup): {
-           goto -> bb7;                     // scope 0 at $DIR/derefer_inline_test.rs:10:11: 10:12
68   }
69   


thread '[mir-opt] src/test/mir-opt/derefer_inline_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_inline_test.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3420:25


failures:
    [mir-opt] src/test/mir-opt/derefer_inline_test.rs
