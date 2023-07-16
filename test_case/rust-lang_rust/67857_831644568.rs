diff
diff --git a/src/librustdoc/passes/check_code_block_syntax.rs b/src/librustdoc/passes/check_code_block_syntax.rs
index 74d396efd57..11c59d6326a 100644
--- a/src/librustdoc/passes/check_code_block_syntax.rs
+++ b/src/librustdoc/passes/check_code_block_syntax.rs
@@ -111,11 +111,14 @@ fn check_rust_syntax(&self, item: &clean::Item, dox: &str, code_block: RustCodeB
                 diag.help(&format!("{}: 