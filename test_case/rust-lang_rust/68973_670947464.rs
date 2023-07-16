diff
--- a/src/librustc_target/spec/windows_gnu_base.rs
+++ b/src/librustc_target/spec/windows_gnu_base.rs
@@ -35,6 +35,8 @@
             "-lmsvcrt".to_string(),
             "-luser32".to_string(),
             "-lkernel32".to_string(),
+            "-lssp_nonshared".to_string(),
+            "-lssp".to_string(),
         ],
     );
     late_link_args_dynamic.insert(
