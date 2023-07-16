patch
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 3fc3b68fd..2622b0785 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -52,7 +52,10 @@ impl Step for ToolBuild {
         let is_optional_tool = self.is_optional_tool;
 
         match self.mode {
-            Mode::ToolRustc => builder.ensure(compile::Rustc { compiler, target }),
+            Mode::ToolRustc => {
+                builder.ensure(compile::Std { compiler, target: compiler.host });
+                builder.ensure(compile::Rustc { compiler, target });
+            },
             Mode::ToolStd => builder.ensure(compile::Std { compiler, target }),
             Mode::ToolBootstrap => {} // uses downloaded stage0 compiler libs
             _ => panic!("unexpected Mode for tool build"),
