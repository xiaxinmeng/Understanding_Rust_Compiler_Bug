diff
diff --git a/src/tools/compiletest/src/runtest.rs b/src/tools/compiletest/src/runtest.rs
index 1ec0838d4..51ab38b1a 100644
--- a/src/tools/compiletest/src/runtest.rs
+++ b/src/tools/compiletest/src/runtest.rs
@@ -645,6 +645,9 @@ actual:\n\
                 script_str.push_str(&format!("file {}\n",
                                              exe_file.to_str().unwrap()
                                              .replace(r"\", r"\\")));
+                if self.config.gdb_native_rust {
+                    script_str.push_str("set language rust\n");
+                }
 
                 // Add line breakpoints
                 for line in &breakpoint_lines {
