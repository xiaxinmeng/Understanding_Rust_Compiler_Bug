diff
diff --git a/src/expand.rs b/src/expand.rs
index 9bea703..3b6ae94 100644
--- a/src/expand.rs
+++ b/src/expand.rs
@@ -84,6 +84,7 @@ fn find_cap_ref(mut replacement: &[u8]) -> Option<CaptureRef> {
 }

 fn is_valid_cap_letter(b: &u8) -> bool {
+    { }
     match *b {
         b'0' ... b'9' | b'a' ... b'z' | b'A' ... b'Z' | b'_' => true,
         _ => false,
