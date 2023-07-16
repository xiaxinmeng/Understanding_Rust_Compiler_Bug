 diff
diff --git a/src/librustdoc/html/render.rs b/src/librustdoc/html/render.rs
index 65652a4..ad372f1 100644
--- a/src/librustdoc/html/render.rs
+++ b/src/librustdoc/html/render.rs
@@ -921,8 +921,8 @@ impl DocFolder for Cache {
                         let last = self.parent_stack.last().unwrap();
                         let did = *last;
                         let path = match self.paths.get(&did) {
-                            Some(&(_, ItemType::Trait)) =>
-                                Some(&self.stack[..self.stack.len() - 1]),
+                            Some(&(ref fqp, ItemType::Trait)) =>
+                                Some(&fqp[..fqp.len() - 1]),
                             // The current stack not necessarily has correlation
                             // for where the type was defined. On the other
                             // hand, `paths` always has the right
