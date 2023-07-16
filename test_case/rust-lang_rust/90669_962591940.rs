diff
--- a/library/std/src/fs/tests.rs
+++ b/library/std/src/fs/tests.rs
@@ -835,17 +835,17 @@ fn read_link() {
         // directory symlink
         assert_eq!(
             check!(fs::read_link(r"C:\Users\All Users")).to_str().unwrap(),
-            r"C:\ProgramData"
+            r"c:\ProgramData"
         );
         // junction
         assert_eq!(
             check!(fs::read_link(r"C:\Users\Default User")).to_str().unwrap(),
-            r"C:\Users\Default"
+            r"c:\Users\Default"
         );
         // junction with special permissions
         assert_eq!(
             check!(fs::read_link(r"C:\Documents and Settings\")).to_str().unwrap(),
-            r"C:\Users"
+            r"c:\Users"
         );
     }
     let tmpdir = tmpdir();
