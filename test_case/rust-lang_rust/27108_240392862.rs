 diff
diff --git a/src/libcore/hash/mod.rs b/src/libcore/hash/mod.rs
index 051eb97..85427d0 100644
--- a/src/libcore/hash/mod.rs
+++ b/src/libcore/hash/mod.rs
@@ -328,8 +328,7 @@ mod impls {
     #[stable(feature = "rust1", since = "1.0.0")]
     impl Hash for str {
         fn hash<H: Hasher>(&self, state: &mut H) {
-            state.write(self.as_bytes());
-            state.write_u8(0xff)
+            self.as_bytes().hash(state)
         }
     }
