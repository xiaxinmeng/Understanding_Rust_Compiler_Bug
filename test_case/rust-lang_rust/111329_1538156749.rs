diff
diff --git a/compiler/rustc_interface/src/passes.rs b/compiler/rustc_interface/src/passes.rs
index 6a94d19001e..a5ca209b2bf 100644
--- a/compiler/rustc_interface/src/passes.rs
+++ b/compiler/rustc_interface/src/passes.rs
@@ -517,6 +517,14 @@ fn write_out_deps(
                     files.push(escape_dep_filename(&path.display().to_string()));
                 }
             }
+
+            match std::env::current_exe() {
+                Ok(driver_path) => files.push(escape_dep_filename(&driver_path.display().to_string())),
+                Err(e) => {
+                    #[allow(rustc::untranslatable_diagnostic, rustc::diagnostic_outside_of_impl)] // old enough this changes a lot in later commits
+                    sess.err(format!("failed to determine path to current process: {e}"));
+                }
+            }
         }
 
         let mut file = BufWriter::new(fs::File::create(&deps_filename)?);
