
     pub fn into_inner(mut self) -> Result<W, IntoInnerError<BufWriter<W>>> {
         match self.flush_buf() {
             Err(e) => Err(IntoInnerError(self, e)),
-            Ok(()) => Ok(self.inner.take().unwrap())
+            Ok(()) => {
+                use std::mem::{replace, uninitialized, forget};
+                let inner = replace(&mut self.inner, unsafe { uninitialized() });
+                replace(&mut self.buf, unsafe { uninitialized() });
+                forget(self);
+                Ok(inner)
+            }
         }
     }
