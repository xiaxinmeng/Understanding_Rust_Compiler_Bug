diff
diff --git procedural-masquerade/lib.rs procedural-masquerade/lib.rs
index 2736ccf..c1a99a1 100644
--- procedural-masquerade/lib.rs
+++ procedural-masquerade/lib.rs
@@ -199,14 +199,23 @@ pub fn _extract_input(derive_input: &str) -> &str {
     let mut input = derive_input;
 
     for expected in &[
-        "#[allow(unused)]",
+        "#",
+        "[",
+        "allow",
+        "(",
+        "unused",
+        ")",
+        "]",
         "enum",
         "ProceduralMasqueradeDummyType",
         "{",
         "Input",
         "=",
-        "(0,",
-        "stringify!",
+        "(",
+        "0",
+        ",",
+        "stringify",
+        "!",
         "(",
     ] {
         input = input.trim_start();
@@ -219,7 +228,7 @@ pub fn _extract_input(derive_input: &str) -> &str {
         input = &input[expected.len()..];
     }
 
-    for expected in [")", ").0,", "}"].iter().rev() {
+    for expected in [")", ")", ".", "0", ",", "}"].iter().rev() {
         input = input.trim_end();
         assert!(
             input.ends_with(expected),
