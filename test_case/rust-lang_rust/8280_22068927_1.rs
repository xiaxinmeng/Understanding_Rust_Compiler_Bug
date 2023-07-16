 diff
-            Some(&x) => map.insert(char, x + 1),
-            None => map.insert(char, 1)
+            None => map.insert(char, 1),
+            Some(&x) => map.insert(char, x + 1),
