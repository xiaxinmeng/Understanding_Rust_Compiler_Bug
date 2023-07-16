diff
-    .map(|children| children.iter().map(|&i| P::from_usize(i))),
+    .map(|children| children.iter().cloned().map(|i| P::from_usize(i))),
