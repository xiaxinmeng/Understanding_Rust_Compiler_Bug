patch
diff --git a/library/std/src/panicking.rs b/library/std/src/panicking.rs
index 4b07b393a2f..c0323fb4d07 100644
--- a/library/std/src/panicking.rs
+++ b/library/std/src/panicking.rs
@@ -319,14 +319,18 @@ pub mod panic_count {
     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
 
     pub fn increase() -> (bool, usize) {
-        (
-            GLOBAL_PANIC_COUNT.fetch_add(1, Ordering::Relaxed) & ALWAYS_ABORT_FLAG != 0,
+        let global_count = GLOBAL_PANIC_COUNT.fetch_add(1, Ordering::Relaxed);
+        let must_abort = global_count & ALWAYS_ABORT_FLAG != 0;
+        let panics = if must_abort {
+            global_count & !ALWAYS_ABORT_FLAG
+        } else {
             LOCAL_PANIC_COUNT.with(|c| {
                 let next = c.get() + 1;
                 c.set(next);
                 next
-            }),
-        )
+            })
+        };
+        (must_abort, panics)
     }
 
     pub fn decrease() {
