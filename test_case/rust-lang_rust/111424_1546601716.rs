plain
test [mir-opt] tests/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/inline/unsized_argument.rs stdout ----
16           StorageLive(_3);                 // scope 0 at $DIR/unsized_argument.rs:+1:13: +1:14
17           _3 = move _1;                    // scope 0 at $DIR/unsized_argument.rs:+1:13: +1:14
18           _7 = (((_3.0: std::ptr::Unique<[i32]>).0: std::ptr::NonNull<[i32]>).0: *const [i32]); // scope 0 at $DIR/unsized_argument.rs:+1:5: +1:15
-           _2 = callee(move (*_7)) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/unsized_argument.rs:+1:5: +1:15
+           _2 = callee(move (*_7)) -> [return: bb2, unwind unreachable]; // scope 0 at $DIR/unsized_argument.rs:+1:5: +1:15
20                                            // mir::Constant
21                                            // + span: $DIR/unsized_argument.rs:8:5: 8:11
22                                            // + literal: Const { ty: fn([i32]) {callee}, val: Value(<ZST>) }

29           return;                          // scope 0 at $DIR/unsized_argument.rs:+2:2: +2:2
31   
31   
-       bb2 (cleanup): {
-           resume;                          // scope 0 at $DIR/unsized_argument.rs:+0:1: +2:2
-   
-       bb3: {
-       bb3: {
-           _4 = alloc::alloc::box_free::<[i32], std::alloc::Global>(move (_3.0: std::ptr::Unique<[i32]>), move (_3.1: std::alloc::Global)) -> bb1; // scope 0 at $DIR/unsized_argument.rs:+1:14: +1:15
-                                            // mir::Constant
-                                            // + span: $DIR/unsized_argument.rs:8:14: 8:15
-                                            // + literal: Const { ty: unsafe fn(Unique<[i32]>, std::alloc::Global) {alloc::alloc::box_free::<[i32], std::alloc::Global>}, val: Value(<ZST>) }
-   
-   
-       bb4 (cleanup): {
-           _6 = alloc::alloc::box_free::<[i32], std::alloc::Global>(move (_3.0: std::ptr::Unique<[i32]>), move (_3.1: std::alloc::Global)) -> [return: bb2, unwind terminate]; // scope 0 at $DIR/unsized_argument.rs:+1:14: +1:15
+       bb2: {
+           _4 = alloc::alloc::box_free::<[i32], std::alloc::Global>(move (_3.0: std::ptr::Unique<[i32]>), move (_3.1: std::alloc::Global)) -> [return: bb1, unwind unreachable]; // scope 0 at $DIR/unsized_argument.rs:+1:14: +1:15
45                                            // mir::Constant
46                                            // + span: $DIR/unsized_argument.rs:8:14: 8:15
47                                            // + literal: Const { ty: unsafe fn(Unique<[i32]>, std::alloc::Global) {alloc::alloc::box_free::<[i32], std::alloc::Global>}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/inline/unsized_argument.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/unsized_argument.caller.Inline.diff', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/inline/unsized_argument.rs
