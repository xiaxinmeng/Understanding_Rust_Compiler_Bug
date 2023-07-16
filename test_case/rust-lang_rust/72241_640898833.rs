diff
index f556bf03f02..744f26239ca 100644
--- a/src/librustc_target/spec/windows_gnu_base.rs
+++ b/src/librustc_target/spec/windows_gnu_base.rs
@@ -20,9 +20,9 @@ pub fn opts() -> TargetOptions {
     late_link_args.insert(
         LinkerFlavor::Gcc,
         vec![
+            "-lmsvcrt".to_string(),
             "-lmingwex".to_string(),
             "-lmingw32".to_string(),
-            "-lmsvcrt".to_string(),
             // mingw's msvcrt is a weird hybrid import library and static library.
             // And it seems that the linker fails to use import symbols from msvcrt
             // that are required from functions in msvcrt in certain cases. For example
