diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 4f88b5854b6..74419605ec6 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -1378,7 +1378,12 @@ pub fn cargo(
         }
 
         // Enable usage of unstable features
-        cargo.env("RUSTC_BOOTSTRAP", "1");
+        match mode {
+            Mode::ToolBootstrap => {}
+            Mode::Std | Mode::Rustc | Mode::ToolStd | Mode::Codegen | Mode::ToolRustc => {
+                cargo.env("RUSTC_BOOTSTRAP", "1");
+            }
+        }
         self.add_rust_test_threads(&mut cargo);
 
         // Almost all of the crates that we compile as part of the bootstrap may
