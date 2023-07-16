diff
diff --git a/components/style/font_face.rs b/components/style/font_face.rs
index 70f904c17a..a3d0ca3882 100644
--- a/components/style/font_face.rs
+++ b/components/style/font_face.rs
@@ -62,7 +62,10 @@ impl ToCss for FontFaceRule {
     // Serialization of FontFaceRule is not specced.
     fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
         try!(dest.write_str("@font-face { font-family: "));
-        try!(self.family.to_css(dest));
+        match self.family.to_css(dest) {
+            Err(err) => return Err(err),
+            Ok(_) => {}
+        };
         try!(dest.write_str(";"));
 
         if self.sources.len() > 0 {
