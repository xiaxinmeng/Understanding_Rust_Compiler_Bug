
diff --git a/src/librustpkg/package_source.rs b/src/librustpkg/package_source.rs
index c2fddaf..f693a49 100644
--- a/src/librustpkg/package_source.rs
+++ b/src/librustpkg/package_source.rs
@@ -202,7 +202,7 @@ impl PkgSrc {
             // Since the operation succeeded, move clone_target to local.
             // First, create all ancestor directories.
             if make_dir_rwx_recursive(&local.pop())
-                && os::rename_file(&clone_target, local) {
+                && os::copy_file(&clone_target, local) {
                  Some(local.clone())
             }
             else {
