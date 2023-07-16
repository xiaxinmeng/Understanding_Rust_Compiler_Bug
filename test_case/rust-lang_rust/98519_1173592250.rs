plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..F
failures:

---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
1 // MIR for `Test::X` 0 mir_map
2 
3 fn Test::X(_1: usize) -> Test {
-     let mut _0: Test;                    // return place in scope 0 at $DIR/unusual-item-types.rs:16:5: 16:13
+     let mut _0: Test;                    // return place in scope 0 at $DIR/unusual-item-types.rs:16:5: 16:6
6     bb0: {
6     bb0: {
-         Deinit(_0);                      // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:13
-         ((_0 as X).0: usize) = move _1;  // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:13
-         discriminant(_0) = 0;            // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:13
-         return;                          // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:13
+         Deinit(_0);                      // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:6
+         ((_0 as X).0: usize) = move _1;  // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:6
+         discriminant(_0) = 0;            // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:6
+         return;                          // scope 0 at $DIR/unusual-item-types.rs:16:5: 16:6
12 }
13 


thread '[mir-opt] src/test/mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.Test-X-{constructor#0}.mir_map.0.64bit.mir', src/tools/compiletest/src/runtest.rs:3441:25


failures:
    [mir-opt] src/test/mir-opt/unusual-item-types.rs
