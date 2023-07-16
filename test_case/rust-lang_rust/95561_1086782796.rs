diff
diff --git a/build.rs b/build.rs
index 4788fa6b0..977a0a518 100644
--- a/build.rs
+++ b/build.rs
@@ -22,6 +22,13 @@ impl TreeSitterParser {
             }
         }

+        let mut build = cc::Build::new();
+        build.include(&dir).warnings(false); // ignore unused parameter warnings
+        for file in c_files {
+            build.file(dir.join(file));
+        }
+        build.compile(self.name);
+
         if !cpp_files.is_empty() {
             let mut cpp_build = cc::Build::new();
             cpp_build
@@ -42,13 +49,6 @@ impl TreeSitterParser {
             }
             cpp_build.compile(&format!("{}-cpp", self.name));
         }
-
-        let mut build = cc::Build::new();
-        build.include(&dir).warnings(false); // ignore unused parameter warnings
-        for file in c_files {
-            build.file(dir.join(file));
-        }
-        build.compile(self.name);
     }
 }
