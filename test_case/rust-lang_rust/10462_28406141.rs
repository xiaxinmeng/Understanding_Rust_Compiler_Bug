 Diff
 pub struct TempDir {
     priv path: Option<Path>
+    priv tmpdir: ~Path
 }
...

     pub fn new_in(tmpdir: &Path, suffix: &str) -> Option<TempDir> {
...
-                Ok(()) => return Some(TempDir { path: Some(p) })
+                Ok(()) => return Some(TempDir { path: Some(p), tmpdir: tmpdir.clone() })
...
     }
...
 impl Drop for TempDir {
     fn drop(&mut self) {
        for path in self.path.iter() {
            if path.exists() {
+               os::change_dir(tmpdir);
                fs::rmdir_recursive(path);
            }
        }
     }
