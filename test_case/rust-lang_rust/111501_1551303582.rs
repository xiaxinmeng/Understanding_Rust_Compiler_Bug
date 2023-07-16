plain
failures:

---- [mir-opt] tests/mir-opt/const_prop/transmute.rs stdout ----
3   
4   fn unreachable_direct() -> ! {
5       let mut _0: !;                       // return place in scope 0 at $DIR/transmute.rs:+0:39: +0:40
-       let mut _1: !;                       // in scope 0 at $DIR/transmute.rs:+0:41: +3:2
-       let _2: Never;                       // in scope 0 at $DIR/transmute.rs:+1:9: +1:10
-       let mut _3: ();                      // in scope 0 at $DIR/transmute.rs:+1:39: +1:41
-       let mut _4: !;                       // in scope 0 at $DIR/transmute.rs:+2:5: +2:15
+       let _1: Never;                       // in scope 0 at $DIR/transmute.rs:+1:9: +1:10
+       let mut _2: ();                      // in scope 0 at $DIR/transmute.rs:+1:39: +1:41
10       scope 1 {
-           debug x => _2;                   // in scope 1 at $DIR/transmute.rs:+1:9: +1:10
+           debug x => _1;                   // in scope 1 at $DIR/transmute.rs:+1:9: +1:10
13       scope 2 {
14       }

15   
15   
16       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/transmute.rs:+0:41: +3:2
-           StorageLive(_2);                 // scope 0 at $DIR/transmute.rs:+1:9: +1:10
-           StorageLive(_3);                 // scope 2 at $DIR/transmute.rs:+1:39: +1:41
-           _3 = ();                         // scope 2 at $DIR/transmute.rs:+1:39: +1:41
-           _2 = move _3 as Never (Transmute); // scope 2 at $DIR/transmute.rs:+1:29: +1:42
+           StorageLive(_1);                 // scope 0 at $DIR/transmute.rs:+1:9: +1:10
Build completed unsuccessfully in 0:01:26
+           StorageLive(_2);                 // scope 2 at $DIR/transmute.rs:+1:39: +1:41
+           _2 = ();                         // scope 2 at $DIR/transmute.rs:+1:39: +1:41
+           _1 = move _2 as Never (Transmute); // scope 2 at $DIR/transmute.rs:+1:29: +1:42
22           unreachable;                     // scope 2 at $DIR/transmute.rs:+1:29: +1:42
24   }


thread '[mir-opt] tests/mir-opt/const_prop/transmute.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/transmute.unreachable_direct.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/const_prop/transmute.rs
