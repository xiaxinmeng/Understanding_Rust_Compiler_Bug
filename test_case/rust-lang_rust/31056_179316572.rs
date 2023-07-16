 diff
diff --git a/src/libstd/sys/windows/process.rs b/src/libstd/sys/windows/process.rs
index 758044c..61cf28b 100644
--- a/src/libstd/sys/windows/process.rs
+++ b/src/libstd/sys/windows/process.rs
@@ -419,11 +419,12 @@ mod tests {
     #[test]
     fn test_make_command_line() {
         fn test_wrapper(prog: &str, args: &[&str]) -> String {
-            String::from_utf16(
-                &make_command_line(OsStr::new(prog),
-                                   &args.iter()
-                                        .map(|a| OsString::from(a))
-                                        .collect::<Vec<OsString>>())).unwrap()
+            let command_line = &make_command_line(OsStr::new(prog),
+                                                  &args.iter()
+                                                       .map(|a| OsString::from(a))
+                                                       .collect::<Vec<OsString>>())
+                                    .unwrap();
+            String::from_utf16(command_line).unwrap()
         }

         assert_eq!(
