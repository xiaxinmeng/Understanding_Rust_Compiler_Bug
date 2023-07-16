diff
-    for b in input {
-        output.push(b as i32);
+    for b in &input {
+        output.push(*b as i32);
    }
