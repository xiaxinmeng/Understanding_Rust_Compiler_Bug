 diff
diff --git a/src/libstd/rc.rs b/src/libstd/rc.rs
index 7d0ddb2..79d6d68 100644
--- a/src/libstd/rc.rs
+++ b/src/libstd/rc.rs
@@ -78,8 +78,12 @@ impl<T> Drop for Rc<T> {
     fn drop(&mut self) {
+        info!("Rc: dropping {}", self.ptr);
         unsafe {
             if self.ptr != 0 as *mut RcBox<T> {
+                info!("Rc: strong = {}, weak = {}", (*self.ptr).strong, (*self.ptr).weak);
                 (*self.ptr).strong -= 1;
                 if (*self.ptr).strong == 0 {
+                    info!("Rc: destroying");
                     read_ptr(self.borrow()); // destroy the contained object
                     if (*self.ptr).weak == 0 {
+                        info!("Rc: freeing");
                         exchange_free(self.ptr as *u8)
@@ -155,6 +159,9 @@ impl<T> Drop for Weak<T> {
     fn drop(&mut self) {
+        info!("Weak: dropping {}", self.ptr);
         unsafe {
             if self.ptr != 0 as *mut RcBox<T> {
+                info!("Weak: strong = {}, weak = {}", (*self.ptr).strong, (*self.ptr).weak);
                 (*self.ptr).weak -= 1;
                 if (*self.ptr).weak == 0 && (*self.ptr).strong == 0 {
+                    info!("Weak: freeing");
                     exchange_free(self.ptr as *u8)
