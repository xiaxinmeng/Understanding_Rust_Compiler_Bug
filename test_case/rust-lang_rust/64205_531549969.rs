diff
diff --git a/src/librustc_errors/emitter.rs b/src/librustc_errors/emitter.rs
index 0ce69eecc6..8a3fbb6e09 100644
--- a/src/librustc_errors/emitter.rs
+++ b/src/librustc_errors/emitter.rs
@@ -1185,6 +1185,10 @@ impl EmitterWriter {
             emit_to_destination(&buffer.render(), level, &mut self.dst, self.short_message)?;
             return Ok(());
         };
+        if annotated_files.len() > 1 {
+            let filename = primary_lo.file.name.to_string();
+            panic!("annotated_files > 1 in {}", filename);
+        }
         if let Ok(pos) =
             annotated_files.binary_search_by(|x| x.file.name.cmp(&primary_lo.file.name)) {
             annotated_files.swap(0, pos);
