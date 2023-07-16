diff
-    write!(MutexGuard(&mutex), "").lol();
+    let writer = MutexGuard(&mutex);
+    write!(writer, "").lol();
