diff
diff --git HEAD:/src/bootstrap/bin/rustc.rs INDEX:/src/bootstrap/bin/rustc.rs
index 37336a56d7..182fc0862c 100644
--- HEAD:/src/bootstrap/bin/rustc.rs
+++ INDEX:/src/bootstrap/bin/rustc.rs
@@ -279,6 +279,10 @@ fn main() {
     }
 
     if verbose > 1 {
+        for (i, (k, v)) in env::vars().enumerate() {
+            eprintln!("rustc env[{}]: {} = {}", i, k, v);
+        }
+        eprintln!("rustc directory: {}", env::current_dir().unwrap().display());
         eprintln!("rustc command: {:?}", cmd);
     }
