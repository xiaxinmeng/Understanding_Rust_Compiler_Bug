rust
-    type Cost<'a> = MP::Cost<'a>;
+    type Cost<'a> where Self: 'a = MP::Cost<'a>;
