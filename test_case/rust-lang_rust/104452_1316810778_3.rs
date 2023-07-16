diff
  fn rep() -> [T; 64] {
      let mut _0: [T; 64];                 // return place in scope 0 at $DIR/enum_repeat.rs:+0:13: +0:20
      let mut _1: T;                       // in scope 0 at $DIR/enum_repeat.rs:+1:6: +1:10
  
      bb0: {
          StorageLive(_1);                 // scope 0 at $DIR/enum_repeat.rs:+1:6: +1:10
          Deinit(_1);                      // scope 0 at $DIR/enum_repeat.rs:+1:6: +1:10
          discriminant(_1) = 0;            // scope 0 at $DIR/enum_repeat.rs:+1:6: +1:10
-         _0 = [move _1; 64];              // scope 0 at $DIR/enum_repeat.rs:+1:5: +1:15
+         _0 = [const T::A; 64];           // scope 0 at $DIR/enum_repeat.rs:+1:5: +1:15
+                                          // mir::Constant
+                                          // + span: $DIR/enum_repeat.rs:10:5: 10:15
+                                          // + literal: Const { ty: T, val: Value(Scalar(0x00)) }
          StorageDead(_1);                 // scope 0 at $DIR/enum_repeat.rs:+1:14: +1:15
          return;                          // scope 0 at $DIR/enum_repeat.rs:+2:2: +2:2
      }
  }
