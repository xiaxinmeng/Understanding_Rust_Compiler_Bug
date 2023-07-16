diff
diff --git a/src/tools/clippy/src/driver.rs b/src/tools/clippy/src/driver.rs
index f4f2259ce..5e688d72d 100644
--- a/src/driver.rs
+++ b/src/driver.rs
@@ -323,7 +323,7 @@ pub fn main() {
                 toolchain_path(home, toolchain)
             })
             .or_else(|| {
-                Command::new("rustc")
+                Command::new(env::var("RUSTC").unwrap_or("rustc"))
                     .arg("--print")
                     .arg("sysroot")
                     .output()
