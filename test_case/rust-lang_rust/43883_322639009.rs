diff
diff --git a/src/libstd/fs.rs b/src/libstd/fs.rs
index d678705626..a12f55ad9c 100644
--- a/src/libstd/fs.rs
+++ b/src/libstd/fs.rs
@@ -936,8 +936,9 @@ impl Permissions {
 
     /// Modifies the readonly flag for this set of permissions. If the
     /// `readonly` argument is `true`, using the resulting `Permission` will
-    /// make the file unwritable. Conversely, if it's is `false`, using the
-    /// resulting `Permission` will make the file writable.
+    /// update file permissions to forbid writing. Conversely, if it's `false`,
+    /// using the resulting `Permission` will update file permissions to allow
+    /// writing.
     ///
     /// This operation does **not** modify the filesystem. To modify the
     /// filesystem use the `fs::set_permissions` function.
