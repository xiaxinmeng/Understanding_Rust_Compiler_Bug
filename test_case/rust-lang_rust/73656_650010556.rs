diff
- // MIR for `float_to_exponential_common` before ConstProp
+ // MIR for `float_to_exponential_common` after ConstProp
  
  fn float_to_exponential_common(_1: &mut std::fmt::Formatter, _2: &T, _3: bool) -> std::result::Result<(), std::fmt::Error> {
      debug fmt => _1;                     // in scope 0 at $DIR/funky_arms.rs:11:35: 11:38
      debug num => _2;                     // in scope 0 at $DIR/funky_arms.rs:11:60: 11:63
      debug upper => _3;                   // in scope 0 at $DIR/funky_arms.rs:11:69: 11:74
      let mut _0: std::result::Result<(), std::fmt::Error>; // return place in scope 0 at $DIR/funky_arms.rs:11:85: 11:91
      let _4: bool;                        // in scope 0 at $DIR/funky_arms.rs:15:9: 15:19
      let mut _5: &std::fmt::Formatter;    // in scope 0 at $DIR/funky_arms.rs:15:22: 15:25
      let mut _7: std::option::Option<usize>; // in scope 0 at $DIR/funky_arms.rs:21:30: 21:45
      let mut _8: &std::fmt::Formatter;    // in scope 0 at $DIR/funky_arms.rs:21:30: 21:33
      let mut _9: isize;                   // in scope 0 at $DIR/funky_arms.rs:21:12: 21:27
      let mut _11: &mut std::fmt::Formatter; // in scope 0 at $DIR/funky_arms.rs:23:43: 23:46
      let mut _12: &T;                     // in scope 0 at $DIR/funky_arms.rs:23:48: 23:51
      let mut _13: core::num::flt2dec::Sign; // in scope 0 at $DIR/funky_arms.rs:23:53: 23:57
      let mut _14: usize;                  // in scope 0 at $DIR/funky_arms.rs:23:59: 23:72
      let mut _15: usize;                  // in scope 0 at $DIR/funky_arms.rs:23:59: 23:68
      let mut _16: bool;                   // in scope 0 at $DIR/funky_arms.rs:23:74: 23:79
      let mut _17: &mut std::fmt::Formatter; // in scope 0 at $DIR/funky_arms.rs:25:46: 25:49
      let mut _18: &T;                     // in scope 0 at $DIR/funky_arms.rs:25:51: 25:54
      let mut _19: core::num::flt2dec::Sign; // in scope 0 at $DIR/funky_arms.rs:25:56: 25:60
      let mut _20: bool;                   // in scope 0 at $DIR/funky_arms.rs:25:62: 25:67
      scope 1 {
          debug force_sign => _4;          // in scope 1 at $DIR/funky_arms.rs:15:9: 15:19
          let _6: core::num::flt2dec::Sign; // in scope 1 at $DIR/funky_arms.rs:16:9: 16:13
          scope 2 {
              debug sign => _6;            // in scope 2 at $DIR/funky_arms.rs:16:9: 16:13
              let _10: usize;              // in scope 2 at $DIR/funky_arms.rs:21:17: 21:26
              scope 3 {
                  debug precision => _10;  // in scope 3 at $DIR/funky_arms.rs:21:17: 21:26
              }
          }
      }
  
      bb0: {
          StorageLive(_4);                 // scope 0 at $DIR/funky_arms.rs:15:9: 15:19
          StorageLive(_5);                 // scope 0 at $DIR/funky_arms.rs:15:22: 15:25
          _5 = &(*_1);                     // scope 0 at $DIR/funky_arms.rs:15:22: 15:25
          _4 = const std::fmt::Formatter::sign_plus(move _5) -> bb1; // scope 0 at $DIR/funky_arms.rs:15:22: 15:37
                                           // ty::Const
                                           // + ty: for<'r> fn(&'r std::fmt::Formatter) -> bool {std::fmt::Formatter::sign_plus}
                                           // + val: Value(Scalar(<ZST>))
                                           // mir::Constant
                                           // + span: $DIR/funky_arms.rs:15:26: 15:35
                                           // + literal: Const { ty: for<'r> fn(&'r std::fmt::Formatter) -> bool {std::fmt::Formatter::sign_plus}, val: Value(Scalar(<ZST>)) }
      }
  
      bb1: {
          StorageDead(_5);                 // scope 0 at $DIR/funky_arms.rs:15:36: 15:37
          StorageLive(_6);                 // scope 1 at $DIR/funky_arms.rs:16:9: 16:13
          switchInt(_4) -> [false: bb3, otherwise: bb2]; // scope 1 at $DIR/funky_arms.rs:17:9: 17:14
      }
  
      bb2: {
          discriminant(_6) = 2;            // scope 1 at $DIR/funky_arms.rs:18:17: 18:41
          goto -> bb4;                     // scope 1 at $DIR/funky_arms.rs:16:16: 19:6
      }
  
      bb3: {
          discriminant(_6) = 0;            // scope 1 at $DIR/funky_arms.rs:17:18: 17:38
          goto -> bb4;                     // scope 1 at $DIR/funky_arms.rs:16:16: 19:6
      }
  
      bb4: {
          StorageLive(_7);                 // scope 2 at $DIR/funky_arms.rs:21:30: 21:45
          StorageLive(_8);                 // scope 2 at $DIR/funky_arms.rs:21:30: 21:33
          _8 = &(*_1);                     // scope 2 at $DIR/funky_arms.rs:21:30: 21:33
          _7 = const std::fmt::Formatter::precision(move _8) -> bb5; // scope 2 at $DIR/funky_arms.rs:21:30: 21:45
                                           // ty::Const
                                           // + ty: for<'r> fn(&'r std::fmt::Formatter) -> std::option::Option<usize> {std::fmt::Formatter::precision}
                                           // + val: Value(Scalar(<ZST>))
                                           // mir::Constant
                                           // + span: $DIR/funky_arms.rs:21:34: 21:43
                                           // + literal: Const { ty: for<'r> fn(&'r std::fmt::Formatter) -> std::option::Option<usize> {std::fmt::Formatter::precision}, val: Value(Scalar(<ZST>)) }
      }
  
      bb5: {
          StorageDead(_8);                 // scope 2 at $DIR/funky_arms.rs:21:44: 21:45
          _9 = discriminant(_7);           // scope 2 at $DIR/funky_arms.rs:21:12: 21:27
          switchInt(move _9) -> [1isize: bb7, otherwise: bb6]; // scope 2 at $DIR/funky_arms.rs:21:12: 21:27
      }
  
      bb6: {
          StorageLive(_17);                // scope 2 at $DIR/funky_arms.rs:25:46: 25:49
          _17 = &mut (*_1);                // scope 2 at $DIR/funky_arms.rs:25:46: 25:49
          StorageLive(_18);                // scope 2 at $DIR/funky_arms.rs:25:51: 25:54
          _18 = _2;                        // scope 2 at $DIR/funky_arms.rs:25:51: 25:54
          StorageLive(_19);                // scope 2 at $DIR/funky_arms.rs:25:56: 25:60
-         _19 = _6;                        // scope 2 at $DIR/funky_arms.rs:25:56: 25:60
+         _19 = const core::num::flt2dec::Sign::Minus; // scope 2 at $DIR/funky_arms.rs:25:56: 25:60
+                                          // ty::Const
+                                          // + ty: core::num::flt2dec::Sign
+                                          // + val: Value(Scalar(0x00))
+                                          // mir::Constant
+                                          // + span: $DIR/funky_arms.rs:25:56: 25:60
+                                          // + literal: Const { ty: core::num::flt2dec::Sign, val: Value(Scalar(0x00)) }
          StorageLive(_20);                // scope 2 at $DIR/funky_arms.rs:25:62: 25:67
          _20 = _3;                        // scope 2 at $DIR/funky_arms.rs:25:62: 25:67
-         _0 = const float_to_exponential_common_shortest::<T>(move _17, move _18, move _19, move _20) -> bb9; // scope 2 at $DIR/funky_arms.rs:25:9: 25:68
+         _0 = const float_to_exponential_common_shortest::<T>(move _17, move _18, const core::num::flt2dec::Sign::Minus, move _20) -> bb9; // scope 2 at $DIR/funky_arms.rs:25:9: 25:68
                                           // ty::Const
                                           // + ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}
                                           // + val: Value(Scalar(<ZST>))
                                           // mir::Constant
                                           // + span: $DIR/funky_arms.rs:25:9: 25:45
                                           // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(Scalar(<ZST>)) }
+                                          // ty::Const
+                                          // + ty: core::num::flt2dec::Sign
+                                          // + val: Value(Scalar(0x00))
+                                          // mir::Constant
+                                          // + span: $DIR/funky_arms.rs:25:9: 25:68
+                                          // + literal: Const { ty: core::num::flt2dec::Sign, val: Value(Scalar(0x00)) }
      }
  
      bb7: {
          StorageLive(_10);                // scope 2 at $DIR/funky_arms.rs:21:17: 21:26
          _10 = ((_7 as Some).0: usize);   // scope 2 at $DIR/funky_arms.rs:21:17: 21:26
          StorageLive(_11);                // scope 3 at $DIR/funky_arms.rs:23:43: 23:46
          _11 = &mut (*_1);                // scope 3 at $DIR/funky_arms.rs:23:43: 23:46
          StorageLive(_12);                // scope 3 at $DIR/funky_arms.rs:23:48: 23:51
          _12 = _2;                        // scope 3 at $DIR/funky_arms.rs:23:48: 23:51
          StorageLive(_13);                // scope 3 at $DIR/funky_arms.rs:23:53: 23:57
-         _13 = _6;                        // scope 3 at $DIR/funky_arms.rs:23:53: 23:57
+         _13 = const core::num::flt2dec::Sign::Minus; // scope 3 at $DIR/funky_arms.rs:23:53: 23:57
+                                          // ty::Const
+                                          // + ty: core::num::flt2dec::Sign
+                                          // + val: Value(Scalar(0x00))
+                                          // mir::Constant
+                                          // + span: $DIR/funky_arms.rs:23:53: 23:57
+                                          // + literal: Const { ty: core::num::flt2dec::Sign, val: Value(Scalar(0x00)) }
          StorageLive(_14);                // scope 3 at $DIR/funky_arms.rs:23:59: 23:72
          StorageLive(_15);                // scope 3 at $DIR/funky_arms.rs:23:59: 23:68
          _15 = _10;                       // scope 3 at $DIR/funky_arms.rs:23:59: 23:68
          _14 = Add(move _15, const 1usize); // scope 3 at $DIR/funky_arms.rs:23:59: 23:72
                                           // ty::Const
                                           // + ty: usize
                                           // + val: Value(Scalar(0x0000000000000001))
                                           // mir::Constant
                                           // + span: $DIR/funky_arms.rs:23:71: 23:72
                                           // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
          StorageDead(_15);                // scope 3 at $DIR/funky_arms.rs:23:71: 23:72
          StorageLive(_16);                // scope 3 at $DIR/funky_arms.rs:23:74: 23:79
          _16 = _3;                        // scope 3 at $DIR/funky_arms.rs:23:74: 23:79
-         _0 = const float_to_exponential_common_exact::<T>(move _11, move _12, move _13, move _14, move _16) -> bb8; // scope 3 at $DIR/funky_arms.rs:23:9: 23:80
+         _0 = const float_to_exponential_common_exact::<T>(move _11, move _12, const core::num::flt2dec::Sign::Minus, move _14, move _16) -> bb8; // scope 3 at $DIR/funky_arms.rs:23:9: 23:80
                                           // ty::Const
                                           // + ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, usize, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}
                                           // + val: Value(Scalar(<ZST>))
                                           // mir::Constant
                                           // + span: $DIR/funky_arms.rs:23:9: 23:42
                                           // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, usize, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(Scalar(<ZST>)) }
+                                          // ty::Const
+                                          // + ty: core::num::flt2dec::Sign
+                                          // + val: Value(Scalar(0x00))
+                                          // mir::Constant
+                                          // + span: $DIR/funky_arms.rs:23:9: 23:80
+                                          // + literal: Const { ty: core::num::flt2dec::Sign, val: Value(Scalar(0x00)) }
      }
  
      bb8: {
          StorageDead(_16);                // scope 3 at $DIR/funky_arms.rs:23:79: 23:80
          StorageDead(_14);                // scope 3 at $DIR/funky_arms.rs:23:79: 23:80
          StorageDead(_13);                // scope 3 at $DIR/funky_arms.rs:23:79: 23:80
          StorageDead(_12);                // scope 3 at $DIR/funky_arms.rs:23:79: 23:80
          StorageDead(_11);                // scope 3 at $DIR/funky_arms.rs:23:79: 23:80
          StorageDead(_10);                // scope 2 at $DIR/funky_arms.rs:24:5: 24:6
          goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:21:5: 26:6
      }
  
      bb9: {
          StorageDead(_20);                // scope 2 at $DIR/funky_arms.rs:25:67: 25:68
          StorageDead(_19);                // scope 2 at $DIR/funky_arms.rs:25:67: 25:68
          StorageDead(_18);                // scope 2 at $DIR/funky_arms.rs:25:67: 25:68
          StorageDead(_17);                // scope 2 at $DIR/funky_arms.rs:25:67: 25:68
          goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:21:5: 26:6
      }
  
      bb10: {
          StorageDead(_6);                 // scope 1 at $DIR/funky_arms.rs:27:1: 27:2
          StorageDead(_4);                 // scope 0 at $DIR/funky_arms.rs:27:1: 27:2
          StorageDead(_7);                 // scope 0 at $DIR/funky_arms.rs:27:1: 27:2
          return;                          // scope 0 at $DIR/funky_arms.rs:27:2: 27:2
      }
  }
  

