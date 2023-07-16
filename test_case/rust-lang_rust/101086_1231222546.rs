plain
failures:

---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
5 
6     bb0: {
7         _0 = const 2_i32;                // scope 0 at $DIR/unusual-item-types.rs:+0:38: +0:39
-         return;                          // scope 0 at $DIR/unusual-item-types.rs:+0:5: +0:39
+         return;                          // scope 0 at $DIR/unusual-item-types.rs:+0:5: +0:40
10 }
11 


thread '[mir-opt] src/test/mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.{impl#0}-ASSOCIATED_CONSTANT.mir_map.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/unusual-item-types.rs
