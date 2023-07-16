patch
diff --git a/src/librustdoc/passes/mod.rs b/src/librustdoc/passes/mod.rs
index 641a6df221..9970ac840f 100644
--- a/src/librustdoc/passes/mod.rs
+++ b/src/librustdoc/passes/mod.rs
@@ -409,7 +409,10 @@ crate fn source_span_for_markdown_range(

     'outer: for (line_no, md_line) in md_lines.enumerate() {
         loop {
-            let source_line = src_lines.next().expect("could not find markdown in source");
+            let source_line = match src_lines.next() {
+                Some(line) => line,
+                None => break,
+            };
             match source_line.find(md_line) {
                 Some(offset) => {
                     if line_no == starting_line {
