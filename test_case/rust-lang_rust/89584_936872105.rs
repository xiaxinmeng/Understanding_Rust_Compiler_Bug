diff
--- a/library/std/src/thread/local/tests.rs
+++ b/library/std/src/thread/local/tests.rs
@@ -297,7 +297,7 @@
             .unwrap();
 
         loop {
-            match SYNC_STATE.compare_exchange_weak(
+            match SYNC_STATE.compare_exchange(
                 THREAD1_WAITING,
                 MAIN_THREAD_RENDEZVOUS,
                 Ordering::SeqCst,
