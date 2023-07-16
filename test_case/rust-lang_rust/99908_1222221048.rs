plain
test [mir-opt] src/test/mir-opt/inline/polymorphic-recursion.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
3 fn main::{closure#0}(_1: &[closure@main::{closure#0}], _2: &i32) -> &i32 {
4     debug x => _2;                       // in scope 0 at $DIR/retag.rs:+0:32: +0:33
5     let mut _0: &i32;                    // return place in scope 0 at $DIR/retag.rs:+0:44: +0:48
-     let _3: &i32;                        // in scope 0 at $DIR/retag.rs:41:13: 41:15
+     let _3: &i32;                        // in scope 0 at $DIR/retag.rs:42:13: 42:15
7     scope 1 {
-         debug _y => _3;                  // in scope 1 at $DIR/retag.rs:41:13: 41:15
+         debug _y => _3;                  // in scope 1 at $DIR/retag.rs:42:13: 42:15
10 
11     bb0: {


12         Retag([fn entry] _1);            // scope 0 at $DIR/retag.rs:+0:31: +0:48
13         Retag([fn entry] _2);            // scope 0 at $DIR/retag.rs:+0:31: +0:48
-         StorageLive(_3);                 // scope 0 at $DIR/retag.rs:41:13: 41:15
-         _3 = _2;                         // scope 0 at $DIR/retag.rs:41:18: 41:19
-         Retag(_3);                       // scope 0 at $DIR/retag.rs:41:18: 41:19
-         _0 = _2;                         // scope 1 at $DIR/retag.rs:42:9: 42:10
-         Retag(_0);                       // scope 1 at $DIR/retag.rs:42:9: 42:10
-         StorageDead(_3);                 // scope 0 at $DIR/retag.rs:43:5: 43:6
+         StorageLive(_3);                 // scope 0 at $DIR/retag.rs:42:13: 42:15
+         _3 = _2;                         // scope 0 at $DIR/retag.rs:42:18: 42:19
+         Retag(_3);                       // scope 0 at $DIR/retag.rs:42:18: 42:19
+         _0 = _2;                         // scope 1 at $DIR/retag.rs:43:9: 43:10
+         Retag(_0);                       // scope 1 at $DIR/retag.rs:43:9: 43:10
+         StorageDead(_3);                 // scope 0 at $DIR/retag.rs:44:5: 44:6
20         return;                          // scope 0 at $DIR/retag.rs:+0:48: +0:48
22 }


thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.main-{closure#0}.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3512:25


failures:
    [mir-opt] src/test/mir-opt/retag.rs
