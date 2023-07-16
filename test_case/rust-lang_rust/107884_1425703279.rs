diff
diff --git a/library/std/src/sys/windows/fs.rs b/library/std/src/sys/windows/fs.rs
index 37809803828..f68cf40c065 100644
--- a/library/std/src/sys/windows/fs.rs
+++ b/library/std/src/sys/windows/fs.rs
@@ -1398,10 +1398,10 @@ fn symlink_junction_inner(original: &Path, junction: &Path) -> io::Result<()> {
         // FIXME: this conversion is very hacky
         let v = br"\??\";
         let v = v.iter().map(|x| *x as u16);
-        for c in v.chain(original.as_os_str().encode_wide()) {
+        v.chain(original.as_os_str().encode_wide()).for_each(|c| {
             *buf.add(i) = c;
             i += 1;
-        }
+        });
         *buf.add(i) = 0;
         i += 1;
         (*db).ReparseTag = c::IO_REPARSE_TAG_MOUNT_POINT;
