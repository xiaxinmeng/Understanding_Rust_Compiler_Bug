diff
-    write!(MutexGuard(&mutex), "") /* no semicolon */
+    let writer = MutexGuard(&mutex);
+    write!(writer, "") /* no semicolon */
