diff
diff --git a/src/lib.rs b/src/lib.rs
index d4de24e..81c278f 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1 +1,5 @@
 pub use spirv_std_macros::Image;
+
+#[doc(hidden)]
+/// [spirv_types]
+pub fn workaround_rustdoc_ice_84738() {}
