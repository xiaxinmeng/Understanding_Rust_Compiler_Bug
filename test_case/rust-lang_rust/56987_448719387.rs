diff
diff --git a/src/lib.rs b/src/lib.rs
index 40694726..5cb1a737 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -17,6 +17,8 @@
 // (currently there is no way to opt into sysroot crates w/o `extern crate`)
 #[allow(unused_extern_crates)]
 extern crate rustc_plugin;
+#[allow(unused_extern_crates)]
+extern crate rustc_driver;
 use self::rustc_plugin::Registry;
 
 #[plugin_registrar]
