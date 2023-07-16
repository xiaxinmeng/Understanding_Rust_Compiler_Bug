 rust
diff --git a/src/libstd/rt/kill.rs b/src/libstd/rt/kill.rs
index 6043ae3..93be953 100644
--- a/src/libstd/rt/kill.rs
+++ b/src/libstd/rt/kill.rs
@@ -580,7 +580,8 @@ impl Death {
         do self.on_exit.take().map |on_exit| {
             if success {
                 // We succeeded, but our children might not. Need to wait for them.
-                let mut inner = self.kill_handle.take_unwrap().unwrap();
+                let tmp = self.kill_handle.take_unwrap();
+                let mut inner = tmp.unwrap();
                 if inner.any_child_failed {
                     success = false;
                 } else {
diff --git a/src/libstd/unstable/sync.rs b/src/libstd/unstable/sync.rs
index f3945d8..7505263 100644
--- a/src/libstd/unstable/sync.rs
+++ b/src/libstd/unstable/sync.rs
@@ -26,7 +26,7 @@ use vec;
 /// An atomically reference counted pointer.
 ///
 /// Enforces no shared-memory safety.
-//#[unsafe_no_drop_flag] FIXME: #9758
+#[unsafe_no_drop_flag]
 pub struct UnsafeArc<T> {
     data: *mut ArcData<T>,
 }
