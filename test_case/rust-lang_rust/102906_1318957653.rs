plain

---- [mir-opt] src/test/mir-opt/inline/inline_into_box_place.rs stdout ----
73 -     }
74 - 
75 -     bb5 (cleanup): {
- -         _6 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> [return: bb4, unwind terminate]; // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
+ -         _6 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> [return: bb4, unwind unreachable]; // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
77 -                                          // mir::Constant
78 -                                          // + span: $DIR/inline_into_box_place.rs:8:42: 8:43
79 -                                          // + literal: Const { ty: unsafe fn(Unique<Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>}, val: Value(<ZST>) }

thread '[mir-opt] src/test/mir-opt/inline/inline_into_box_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3447:21


failures:
    [mir-opt] src/test/mir-opt/inline/inline_into_box_place.rs
