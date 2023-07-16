plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
............
failures:

---- [mir-opt] src/test/mir-opt/enum_cast.rs stdout ----
3 fn foo(_1: Foo) -> usize {
4     debug foo => _1;                     // in scope 0 at $DIR/enum_cast.rs:+0:8: +0:11
5     let mut _0: usize;                   // return place in scope 0 at $DIR/enum_cast.rs:+0:21: +0:26
-     let _2: Foo;                         // in scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
-     let mut _3: isize;                   // in scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
+     let mut _2: isize;                   // in scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
+     let _3: Foo;                         // in scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
9     bb0: {
9     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
-         _2 = move _1;                    // scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
-         _3 = discriminant(_2);           // scope 0 at $DIR/enum_cast.rs:+1:5: +1:17
-         _0 = move _3 as usize (IntToInt); // scope 0 at $DIR/enum_cast.rs:+1:5: +1:17
-         StorageDead(_2);                 // scope 0 at $DIR/enum_cast.rs:+1:16: +1:17
+         StorageLive(_3);                 // scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
+         _3 = move _1;                    // scope 0 at $DIR/enum_cast.rs:+1:5: +1:8
+         _2 = discriminant(_3);           // scope 0 at $DIR/enum_cast.rs:+1:5: +1:17
+         _0 = move _2 as usize (IntToInt); // scope 0 at $DIR/enum_cast.rs:+1:5: +1:17
+         StorageDead(_3);                 // scope 0 at $DIR/enum_cast.rs:+1:16: +1:17
15         return;                          // scope 0 at $DIR/enum_cast.rs:+2:2: +2:2
17 }


thread '[mir-opt] src/test/mir-opt/enum_cast.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/enum_cast.foo.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25


failures:
    [mir-opt] src/test/mir-opt/enum_cast.rs
