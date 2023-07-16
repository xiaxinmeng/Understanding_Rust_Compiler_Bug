

---- [mir-opt] mir-opt/inline/inline-shims.rs stdout ----
9	      let mut _4: *mut std::vec::Vec<A>;   // in scope 0 at $DIR/inline-shims.rs:10:38: 10:39
10	      let mut _5: *mut std::option::Option<B>; // in scope 0 at $DIR/inline-shims.rs:11:38: 11:39
11	      scope 1 {
+	+         scope 3 (inlined drop_in_place::<Vec<A>> - shim(Some(Vec<A>))) { // at $DIR/inline-shims.rs:10:14: 10:40
+	+             let mut _6: &mut std::vec::Vec<A>; // in scope 3 at $DIR/inline-shims.rs:10:14: 10:40
+	+             let mut _7: ();              // in scope 3 at $DIR/inline-shims.rs:10:14: 10:40
+	+         }
12	      }
13	      scope 2 {
-	+         scope 3 (inlined drop_in_place::<Option<B>> - shim(Some(Option<B>))) { // at $DIR/inline-shims.rs:11:14: 11:40
-	+             let mut _6: isize;           // in scope 3 at $DIR/inline-shims.rs:11:14: 11:40
-	+             let mut _7: isize;           // in scope 3 at $DIR/inline-shims.rs:11:14: 11:40
+	+         scope 4 (inlined drop_in_place::<Option<B>> - shim(Some(Option<B>))) { // at $DIR/inline-shims.rs:11:14: 11:40
+	+             let mut _8: isize;           // in scope 4 at $DIR/inline-shims.rs:11:14: 11:40
+	+             let mut _9: isize;           // in scope 4 at $DIR/inline-shims.rs:11:14: 11:40
17	+         }
18	      }
19	  

21	          StorageLive(_3);                 // scope 0 at $DIR/inline-shims.rs:10:5: 10:42
22	          StorageLive(_4);                 // scope 1 at $DIR/inline-shims.rs:10:38: 10:39
23	          _4 = _1;                         // scope 1 at $DIR/inline-shims.rs:10:38: 10:39
-	          _3 = drop_in_place::<Vec<A>>(move _4) -> bb1; // scope 1 at $DIR/inline-shims.rs:10:14: 10:40
+	-         _3 = drop_in_place::<Vec<A>>(move _4) -> bb1; // scope 1 at $DIR/inline-shims.rs:10:14: 10:40
+	+         _6 = &mut (*_4);                 // scope 3 at $DIR/inline-shims.rs:10:14: 10:40
+	+         _7 = <Vec<A> as Drop>::drop(move _6) -> bb2; // scope 3 at $DIR/inline-shims.rs:10:14: 10:40
25	                                           // mir::Constant
-	                                           // + span: $DIR/inline-shims.rs:10:14: 10:37
-	                                           // + literal: Const { ty: unsafe fn(*mut std::vec::Vec<A>) {std::intrinsics::drop_in_place::<std::vec::Vec<A>>}, val: Value(Scalar(<ZST>)) }
+	-                                          // + span: $DIR/inline-shims.rs:10:14: 10:37
+	-                                          // + literal: Const { ty: unsafe fn(*mut std::vec::Vec<A>) {std::intrinsics::drop_in_place::<std::vec::Vec<A>>}, val: Value(Scalar(<ZST>)) }
+	+                                          // + span: $DIR/inline-shims.rs:10:14: 10:40
+	+                                          // + literal: Const { ty: for<'r> fn(&'r mut std::vec::Vec<A>) {<std::vec::Vec<A> as std::ops::Drop>::drop}, val: Value(Scalar(<ZST>)) }
28	      }
29	  
30	      bb1: {

36	-                                          // mir::Constant
37	-                                          // + span: $DIR/inline-shims.rs:11:14: 11:37
38	-                                          // + literal: Const { ty: unsafe fn(*mut std::option::Option<B>) {std::intrinsics::drop_in_place::<std::option::Option<B>>}, val: Value(Scalar(<ZST>)) }
-	+         _6 = discriminant((*_5));        // scope 3 at $DIR/inline-shims.rs:11:14: 11:40
-	+         switchInt(move _6) -> [0_isize: bb2, otherwise: bb3]; // scope 3 at $DIR/inline-shims.rs:11:14: 11:40
+	+         _8 = discriminant((*_5));        // scope 4 at $DIR/inline-shims.rs:11:14: 11:40
+	+         switchInt(move _8) -> [0_isize: bb3, otherwise: bb4]; // scope 4 at $DIR/inline-shims.rs:11:14: 11:40
41	      }
42	  
43	      bb2: {

+	+         drop(((*_4).0: alloc::raw_vec::RawVec<A>)) -> bb1; // scope 3 at $DIR/inline-shims.rs:10:14: 10:40
+	+     }
+	+ 
+	+     bb3: {
44	          StorageDead(_5);                 // scope 2 at $DIR/inline-shims.rs:11:39: 11:40
45	          return;                          // scope 0 at $DIR/inline-shims.rs:12:2: 12:2
46	+     }

47	+ 
-	+     bb3: {
-	+         drop((((*_5) as Some).0: B)) -> bb2; // scope 3 at $DIR/inline-shims.rs:11:14: 11:40
+	+     bb4: {
+	+         drop((((*_5) as Some).0: B)) -> bb3; // scope 4 at $DIR/inline-shims.rs:11:14: 11:40
50	      }
51	  }
52	  

thread '[mir-opt] mir-opt/inline/inline-shims.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_shims.drop.Inline.diff', src/tools/compiletest/src/runtest.rs:3272:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
