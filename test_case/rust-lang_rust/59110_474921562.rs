
diff --git a/src/bootstrap/bin/rustc.rs b/src/bootstrap/bin/rustc.rs
index ca86aeb810..b884d815bf 100644
--- a/src/bootstrap/bin/rustc.rs
+++ b/src/bootstrap/bin/rustc.rs
@@ -111,7 +111,7 @@ fn main() {
 
         // Link crates to the proc macro crate for the target, but use a host proc macro crate
         // to actually run the macros
-        if env::var_os("RUST_DUAL_PROC_MACROS").is_some() {
+        if env::var_os("RUST_DUAL_PROC_MACROS").is_some() && stage != "0" {
             cmd.arg("-Zdual-proc-macros");
         }
