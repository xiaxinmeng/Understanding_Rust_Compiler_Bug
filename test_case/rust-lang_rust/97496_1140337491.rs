plain
test [mir-opt] src/test/mir-opt/uniform_array_move_out.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/derefer_inline_test.rs stdout ----
25       bb1: {
26           StorageLive(_5);                 // scope 0 at $DIR/derefer_inline_test.rs:8:5: 8:12
27           _5 = ShallowInitBox(move _4, std::boxed::Box<i32>); // scope 0 at $DIR/derefer_inline_test.rs:8:5: 8:12
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
-           (*_5) = f() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/derefer_inline_test.rs:8:9: 8:12
+           (*_5) = f() -> bb2;              // scope 0 at $DIR/derefer_inline_test.rs:8:9: 8:12
29                                            // mir::Constant
30                                            // + span: $DIR/derefer_inline_test.rs:8:9: 8:10
31                                            // + literal: Const { ty: fn() -> Box<i32> {f}, val: Value(Scalar(<ZST>)) }
38   
39       bb3: {
39       bb3: {
40           StorageDead(_5);                 // scope 0 at $DIR/derefer_inline_test.rs:8:11: 8:12
-           drop(_1) -> [return: bb4, unwind: bb6]; // scope 0 at $DIR/derefer_inline_test.rs:8:12: 8:13
+           drop(_1) -> bb4;                 // scope 0 at $DIR/derefer_inline_test.rs:8:12: 8:13
43   
44       bb4: {

48       }
48       }
49   
50       bb5 (cleanup): {
-           goto -> bb8;                     // scope 0 at $DIR/derefer_inline_test.rs:8:11: 8:12
-   
-   
-       bb6 (cleanup): {
55           resume;                          // scope 0 at $DIR/derefer_inline_test.rs:7:1: 9:2
-   
-   
-       bb7 (cleanup): {
-           _6 = alloc::alloc::box_free::<Box<i32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::boxed::Box<i32>>), move (_5.1: std::alloc::Global)) -> bb6; // scope 0 at $DIR/derefer_inline_test.rs:8:11: 8:12
-                                            // mir::Constant
-                                            // + span: $DIR/derefer_inline_test.rs:8:11: 8:12
-                                            // + literal: Const { ty: unsafe fn(Unique<Box<i32>>, std::alloc::Global) {alloc::alloc::box_free::<Box<i32>, std::alloc::Global>}, val: Value(Scalar(<ZST>)) }
-   
-   
-       bb8 (cleanup): {
-           goto -> bb7;                     // scope 0 at $DIR/derefer_inline_test.rs:8:11: 8:12
68   }
69   


thread '[mir-opt] src/test/mir-opt/derefer_inline_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_inline_test.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3420:25


failures:
    [mir-opt] src/test/mir-opt/derefer_inline_test.rs
