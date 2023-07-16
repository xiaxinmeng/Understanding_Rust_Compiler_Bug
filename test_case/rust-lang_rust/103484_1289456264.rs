diff
diff --git a/src/tools/lint-docs/src/lib.rs b/src/tools/lint-docs/src/lib.rs
index 857feb77325..3842a649c6f 100644
--- a/src/tools/lint-docs/src/lib.rs
+++ b/src/tools/lint-docs/src/lib.rs
@@ -37,10 +37,8 @@ fn doc_contains(&self, text: &str) -> bool {
     }

     fn is_ignored(&self) -> bool {
-        self.doc
-            .iter()
-            .filter(|line| line.starts_with("