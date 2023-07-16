diff
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 24b033cc0dc..b8ea072e584 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -70,7 +70,17 @@ fn run(self, builder: &Builder<'_>) -> Option<PathBuf> {
             &self.extra_features,
         );
 
-        builder.info(&format!("Building stage{} tool {} ({})", compiler.stage, tool, target));
+        let msg = if self.mode == Mode::ToolRustc {
+            format!(
+                "Building stage {} tool {tool} using stage {} sysroot ({target})",
+                compiler.stage + 1,
+                compiler.stage
+            )
+        } else {
+            format!("Building tool {tool} using stage {} sysroot ({target})", compiler.stage)
+        };
+        builder.info(&msg);
+
         let mut duplicates = Vec::new();
         let is_expected = compile::stream_cargo(builder, cargo, vec![], &mut |msg| {
             // Only care about big things like the RLS/Cargo for now
