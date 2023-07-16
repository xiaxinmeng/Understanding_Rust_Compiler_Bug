diff
-    let closure = |x| Ref(x);
+    let closure: for<'a> fn(&'a ()) -> Ref<'a> = |x| Ref(x);
