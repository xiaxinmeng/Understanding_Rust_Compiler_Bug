plain

---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
26     }
27 
28     bb5: {
-         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> [return: bb3, unwind: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         drop(((*_1).0: std::boxed::Box<[std::mem::MaybeUninit<i32>]>)) -> [return: bb3, unwind: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
31 
32     bb6: {


thread '[mir-opt] src/test/mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.64bit.mir', src/tools/compiletest/src/runtest.rs:3406:25


failures:
    [mir-opt] src/test/mir-opt/unusual-item-types.rs
