diff
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 5c874f69bd9..c82b78c1c71 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -564,6 +564,7 @@ pub struct Cargo {
 impl Step for Cargo {
     type Output = PathBuf;
     const DEFAULT: bool = true;
+    const ENABLE_DOWNLOAD_RUSTC: bool = true;
     const ONLY_HOSTS: bool = true;
 
     fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
@@ -647,6 +648,7 @@ pub struct $name {
         impl Step for $name {
             type Output = Option<PathBuf>;
             const DEFAULT: bool = true; // Overwritten below
+            const ENABLE_DOWNLOAD_RUSTC: bool = true;
             const ONLY_HOSTS: bool = true;
 
             fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
