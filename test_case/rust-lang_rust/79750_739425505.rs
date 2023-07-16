diff
diff --git a/src/tools/lint-docs/src/lib.rs b/src/tools/lint-docs/src/lib.rs
index 326b7948098..ea54a351e03 100644
--- a/src/tools/lint-docs/src/lib.rs
+++ b/src/tools/lint-docs/src/lib.rs
@@ -143,8 +143,8 @@ fn lints_from_file(&self, path: &Path) -> Result<Vec<Lint>, Box<dyn Error>> {
                     Some((lineno, line)) => {
                         let line = line.trim();
                         if let Some(text) = line.strip_prefix("/// ") {
-                            doc_lines.push(text.trim().to_string());
-                        } else if line.starts_with("///") {
+                            doc_lines.push(text.to_string());
+                        } else if line == "///" {
                             doc_lines.push("".to_string());
                         } else if line.starts_with("// ") {
                             // Ignore comments.
