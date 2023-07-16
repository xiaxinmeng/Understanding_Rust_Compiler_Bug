 udiff
diff --git a/src/libcore/repr.rs b/src/libcore/repr.rs
index 7f1687b..5055b7b 100644
--- a/src/libcore/repr.rs
+++ b/src/libcore/repr.rs
@@ -448,7 +448,9 @@ impl TyVisitor for ReprVisitor {
         true
     }
     fn visit_leave_tup(&self, _n_fields: uint,
-                       _sz: uint, _align: uint) -> bool {
+                       sz: uint, _align: uint) -> bool {
+        if (sz == 1)
+            self.writer.write_char(',');
         self.writer.write_char(')');
         true
     }
