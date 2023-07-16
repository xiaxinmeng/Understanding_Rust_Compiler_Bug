diff
diff --git a/src/librustdoc/html/render/write_shared.rs b/src/librustdoc/html/render/write_shared.rs
index 94d8a9feca6..38a9af99568 100644
--- a/src/librustdoc/html/render/write_shared.rs
+++ b/src/librustdoc/html/render/write_shared.rs
@@ -137,7 +137,7 @@ fn collect(path: &Path, krate: &str) -> io::Result<(Vec<String>, Vec<String>)> {
         Ok((ret, krates))
     }
 
-    /// Read a file and return all lines that match the <code>"{crate}":{data},\</code> format,
+    /// Read a file and return all lines that match the <code> "{crate}":{data},\ </code> format,
     /// and return a tuple `(Vec<DataString>, Vec<CrateNameString>)`.
     ///
     /// This forms the payload of files that look like this:
