diff
@@ -403,7 +405,9 @@ impl<T, A: Alloc> RawVec<T, A> {
     /// # Aborts
     ///
     /// Aborts on OOM
-    pub fn reserve_exact(&mut self, used_cap: usize, needed_extra_cap: usize) {
+    pub fn try_reserve_exact(&mut self, used_cap: usize, needed_extra_cap: usize)
+           -> Result<(), CollectionAllocErr> {
+
         unsafe {
             // NOTE: we don't early branch on ZSTs here because we want this
             // to actually catch "asking for more than usize::MAX" in that case.
@@ -413,16 +417,15 @@ impl<T, A: Alloc> RawVec<T, A> {
             // Don't actually need any more capacity.
             // Wrapping in case they gave a bad `used_cap`.
             if self.cap().wrapping_sub(used_cap) >= needed_extra_cap {
-                return;
+                return Ok(());
             }
 
             // Nothing we can really do about these checks :(
-            let new_cap = used_cap.checked_add(needed_extra_cap).expect("capacity overflow");
-            let new_layout = match Layout::array::<T>(new_cap) {
-                Some(layout) => layout,
-                None => panic!("capacity overflow"),
-            };
-            alloc_guard(new_layout.size());
+            let new_cap = used_cap.checked_add(needed_extra_cap).ok_or(CapacityOverflow)?;
+            let new_layout = Layout::array::<T>(new_cap).ok_or(CapacityOverflow)?;
+
+            alloc_guard(new_layout.size())?;
+
             let res = match self.current_layout() {
                 Some(layout) => {
                     let old_ptr = self.ptr.as_ptr() as *mut u8;
@@ -430,26 +433,34 @@ impl<T, A: Alloc> RawVec<T, A> {
                 }
                 None => self.a.alloc(new_layout),
             };
-            let uniq = match res {
-                Ok(ptr) => Unique::new_unchecked(ptr as *mut T),
-                Err(e) => self.a.oom(e),
-            };
-            self.ptr = uniq;
+
+            self.ptr = Unique::new_unchecked(res? as *mut T);
             self.cap = new_cap;
+
+            Ok(())
         }
     }
 
+    pub fn reserve_exact(&mut self, used_cap: usize, needed_extra_cap: usize) {
+        match self.try_reserve_exact(used_cap, needed_extra_cap) {
+            Err(CapacityOverflow) => panic!("capacity overflow"),
+            Err(AllocErr(e)) => self.a.oom(e),
+            Ok(()) => { /* yay */ }
+         }
+     }
+
