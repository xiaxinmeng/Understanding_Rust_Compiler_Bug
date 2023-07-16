diff
-                let p = self.get_unchecked_mut(index) as *mut _;
+                let p: *mut _ = self.get_unchecked_mut(index);
                 ptr::copy(p, p.offset(1), len - index);
