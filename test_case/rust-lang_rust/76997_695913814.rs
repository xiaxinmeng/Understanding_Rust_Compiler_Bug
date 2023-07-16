patch
diff --git a/src/test/mir-opt/inline/inline_closure_borrows_arg.foo.Inline.after.mir b/src/test/mir-opt/inline/inline_closure_borrows_arg.foo.Inline.after.mir
index 4e61f1c2ef0..26139b02f56 100644
--- a/src/test/mir-opt/inline/inline_closure_borrows_arg.foo.Inline.after.mir
+++ b/src/test/mir-opt/inline/inline_closure_borrows_arg.foo.Inline.after.mir
@@ -16,11 +16,11 @@ fn foo(_1: T, _2: &i32) -> i32 {
         scope 2 (inlined foo::<T>::{{closure}}#0) { // at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
             debug r => _8;               // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:14: 12:15
             debug _s => _9;              // in scope 2 at $DIR/inline-closure-borrows-arg.rs:12:23: 12:25
+            scope 3 {
+                debug variable => _8;    // in scope 3 at $DIR/inline-closure-borrows-arg.rs:13:13: 13:21
+            }
         }
     }
-    scope 3 {
-        debug variable => _8;            // in scope 3 at $DIR/inline-closure-borrows-arg.rs:13:13: 13:21
-    }
 
     bb0: {
         StorageLive(_3);                 // scope 0 at $DIR/inline-closure-borrows-arg.rs:12:9: 12:10
