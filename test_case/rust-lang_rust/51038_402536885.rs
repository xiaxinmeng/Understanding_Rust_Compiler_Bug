diff
diff --git a/src/librustc_driver/lib.rs b/src/librustc_driver/lib.rs
index 84f7b35d21..c7e9fc77ce 100644
--- a/src/librustc_driver/lib.rs
+++ b/src/librustc_driver/lib.rs
@@ -33,6 +33,8 @@ extern crate arena;
 extern crate getopts;
 extern crate graphviz;
 extern crate env_logger;
+#[cfg(not(windows))]
+extern crate jemallocator;
 #[cfg(unix)]
 extern crate libc;
 extern crate rustc_rayon as rayon;
@@ -118,6 +120,11 @@ pub mod driver;
 pub mod pretty;
 mod derive_registrar;
 
+#[cfg(not(windows))]
+#[cfg(not(stage0))]
+#[global_allocator]
+static A: jemallocator::Jemalloc = jemallocator::Jemalloc;
+
 pub mod target_features {
     use syntax::ast;
     use syntax::symbol::Symbol;
