patch
diff --git a/src/bootstrap/bin/rustc.rs b/src/bootstrap/bin/rustc.rs
index 7429492f91..d508969d29 100644
--- a/src/bootstrap/bin/rustc.rs
+++ b/src/bootstrap/bin/rustc.rs
@@ -211,7 +211,7 @@ fn main() {
                 || target.contains("-none-eabi")
                 || target.ends_with("-none-elf"))
         {
-            cmd.arg("-Zemit-stack-sizes");
+            //cmd.arg("-Zemit-stack-sizes");
         }
 
         if let Ok(s) = env::var("RUSTC_CODEGEN_UNITS") {
