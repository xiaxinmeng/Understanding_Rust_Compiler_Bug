
---- [mir-opt] src/test/mir-opt/deref-patterns/string.rs stdout ----
25	        _7 = const false;                // scope 0 at $DIR/string.rs:+3:9: +3:10
26	        _6 = move _1;                    // scope 0 at $DIR/string.rs:+3:9: +3:10
27	        _0 = const 4321_i32;             // scope 1 at $DIR/string.rs:+3:14: +3:18
-	        drop(_6) -> [return: bb6, unwind: bb12]; // scope 0 at $DIR/string.rs:+3:17: +3:18
+	        drop(_6) -> bb6;                 // scope 0 at $DIR/string.rs:+3:17: +3:18
29	    }
30	
31	    bb2: {

37	    }
38	
39	    bb3: {
-	        _4 = <str as PartialEq>::eq(_3, const "a") -> [return: bb4, unwind: bb12]; // scope 0 at $DIR/string.rs:+2:14: +2:17
+	        _4 = <str as PartialEq>::eq(_3, const "a") -> bb4; // scope 0 at $DIR/string.rs:+2:14: +2:17
41	                                         // mir::Constant
42	                                         // + span: $DIR/string.rs:9:14: 9:17
43	                                         // + literal: Const { ty: for<'a, 'b> fn(&'a str, &'b str) -> bool {<str as PartialEq>::eq}, val: Value(<ZST>) }

52	
53	    bb5: {
54	        _0 = const 1234_i32;             // scope 0 at $DIR/string.rs:+2:22: +2:26
-	        goto -> bb10;                    // scope 0 at $DIR/string.rs:+2:22: +2:26
+	        goto -> bb9;                     // scope 0 at $DIR/string.rs:+2:22: +2:26
56	    }
57	
58	    bb6: {

59	        StorageDead(_6);                 // scope 0 at $DIR/string.rs:+3:17: +3:18
-	        goto -> bb10;                    // scope 0 at $DIR/string.rs:+3:17: +3:18
+	        goto -> bb9;                     // scope 0 at $DIR/string.rs:+3:17: +3:18
61	    }
62	
63	    bb7: {

64	        return;                          // scope 0 at $DIR/string.rs:+5:2: +5:2
65	    }
66	
-	    bb8 (cleanup): {
-	        resume;                          // scope 0 at $DIR/string.rs:+0:1: +5:2
+	    bb8: {
+	        drop(_1) -> bb7;                 // scope 0 at $DIR/string.rs:+5:1: +5:2
69	    }
70	
71	    bb9: {

-	        drop(_1) -> [return: bb7, unwind: bb8]; // scope 0 at $DIR/string.rs:+5:1: +5:2
-	    }
-	
-	    bb10: {
-	        switchInt(_7) -> [false: bb7, otherwise: bb9]; // scope 0 at $DIR/string.rs:+5:1: +5:2
-	    }
-	
-	    bb11 (cleanup): {
-	        drop(_1) -> bb8;                 // scope 0 at $DIR/string.rs:+5:1: +5:2
-	    }
-	
-	    bb12 (cleanup): {
-	        switchInt(_7) -> [false: bb8, otherwise: bb11]; // scope 0 at $DIR/string.rs:+5:1: +5:2
+	        switchInt(_7) -> [false: bb7, otherwise: bb8]; // scope 0 at $DIR/string.rs:+5:1: +5:2
85	    }
86	}
87	
