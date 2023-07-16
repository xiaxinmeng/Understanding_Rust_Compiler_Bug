`diff
--- a_string.mir	2023-05-14 11:38:15.942618912 +0200
+++ a.mir	2023-05-14 11:38:23.439260694 +0200
@@ -3,7 +3,7 @@
 fn split(_1: String) -> () {
     debug s => _1;                       // in scope 0 at a.rs:3:14: 3:15
     let mut _0: ();                      // return place in scope 0 at a.rs:3:25: 3:25
-    let _2: std::str::Split<'_, &str>;   // in scope 0 at a.rs:4:9: 4:10
+    let _2: std::str::Split<'_, char>;   // in scope 0 at a.rs:4:9: 4:10
     let mut _3: &str;                    // in scope 0 at a.rs:4:13: 4:25
     let _4: &str;                        // in scope 0 at a.rs:4:13: 4:25
     let mut _5: &std::string::String;    // in scope 0 at a.rs:4:13: 4:25
@@ -21,13 +21,10 @@

     bb1: {
         _3 = _4;                         // scope 0 at a.rs:4:13: 4:25
-        _2 = core::str::<impl str>::split::<'_, &str>(move _3, const "a") -> [return: bb2, unwind: bb4]; // scope 0 at a.rs:4:13: 4:25
+        _2 = core::str::<impl str>::split::<'_, char>(move _3, const 'a') -> [return: bb2, unwind: bb4]; // scope 0 at a.rs:4:13: 4:25
                                          // mir::Constant
                                          // + span: a.rs:4:15: 4:20
-                                         // + literal: Const { ty: fn(&str, &str) -> std::str::Split<'_, &str> {core::str::<impl str>::split::<'_, &str>}, val: Value(<ZST>) }
-                                         // mir::Constant
-                                         // + span: a.rs:4:21: 4:24
-                                         // + literal: Const { ty: &str, val: Value(Slice(..)) }
+                                         // + literal: Const { ty: fn(&str, char) -> std::str::Split<'_, char> {core::str::<impl str>::split::<'_, char>}, val: Value(<ZST>) }
     }

     bb2: {
