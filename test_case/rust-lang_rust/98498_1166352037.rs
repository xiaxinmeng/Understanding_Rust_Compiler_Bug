diff
diff --git a/library/std/src/thread/scoped.rs b/library/std/src/thread/scoped.rs
index a387a09dc8b..06019cbc75b 100644
--- a/library/std/src/thread/scoped.rs
+++ b/library/std/src/thread/scoped.rs
@@ -55,8 +55,10 @@ pub(super) fn decrement_num_running_threads(&self, panic: bool) {
         if panic {
             self.a_thread_panicked.store(true, Ordering::Relaxed);
         }
+        // The moment we do the `fetch_sub`, the memory we ourselves are stored in can be deallocated.
+        let main_thread = self.main_thread.clone();
         if self.num_running_threads.fetch_sub(1, Ordering::Release) == 1 {
-            self.main_thread.unpark();
+            main_thread.unpark();
         }
     }
 }
