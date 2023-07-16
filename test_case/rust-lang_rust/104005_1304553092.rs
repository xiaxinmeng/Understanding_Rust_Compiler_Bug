diff
-    let val = extend_lt(&String::from("blah blah blah"));
+    let val = extend_lt::<_, &_>(&String::from("blah blah blah"));
