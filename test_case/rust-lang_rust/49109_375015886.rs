diff
diff --git a/src/libstd/sys_common/wtf8.rs b/src/libstd/sys_common/wtf8.rs
index 175107bdc4..78b2bb5fe6 100644
--- a/src/libstd/sys_common/wtf8.rs
+++ b/src/libstd/sys_common/wtf8.rs
@@ -871,21 +871,21 @@ impl Hash for Wtf8 {
 }
 
 impl Wtf8 {
-    fn is_ascii(&self) -> bool {
+    pub fn is_ascii(&self) -> bool {
         self.bytes.is_ascii()
     }
-    fn to_ascii_uppercase(&self) -> Wtf8Buf {
+    pub fn to_ascii_uppercase(&self) -> Wtf8Buf {
         Wtf8Buf { bytes: self.bytes.to_ascii_uppercase() }
     }
-    fn to_ascii_lowercase(&self) -> Wtf8Buf {
+    pub fn to_ascii_lowercase(&self) -> Wtf8Buf {
         Wtf8Buf { bytes: self.bytes.to_ascii_lowercase() }
     }
-    fn eq_ignore_ascii_case(&self, other: &Wtf8) -> bool {
+    pub fn eq_ignore_ascii_case(&self, other: &Wtf8) -> bool {
         self.bytes.eq_ignore_ascii_case(&other.bytes)
     }
 
-    fn make_ascii_uppercase(&mut self) { self.bytes.make_ascii_uppercase() }
-    fn make_ascii_lowercase(&mut self) { self.bytes.make_ascii_lowercase() }
+    pub fn make_ascii_uppercase(&mut self) { self.bytes.make_ascii_uppercase() }
+    pub fn make_ascii_lowercase(&mut self) { self.bytes.make_ascii_lowercase() }
 }
 
 #[cfg(test)]
