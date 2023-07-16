diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 707c1ff3efa..0f6a74b01ce 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -1114,6 +1114,10 @@ impl<'a> Builder<'a> {
             }
         }
 
+        if mode != Mode::Std {
+            rustflags.arg("-Ztls-model=initial-exec");
+        }
+
         if self.config.incremental {
             cargo.env("CARGO_INCREMENTAL", "1");
         } else {
