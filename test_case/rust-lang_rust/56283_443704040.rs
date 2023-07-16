diff
diff --git a/rayon-demo/Cargo.toml b/rayon-demo/Cargo.toml
index 897b5c8..e1aba34 100644
--- a/rayon-demo/Cargo.toml
+++ b/rayon-demo/Cargo.toml
@@ -5,6 +5,7 @@ authors = ["Niko Matsakis <niko@alum.mit.edu>"]
 publish = false

 [dependencies]
+jemallocator = "0.1.8"
 rayon = { path = "../" }
 cgmath = "0.16"
 docopt = "1"
diff --git a/rayon-demo/src/main.rs b/rayon-demo/src/main.rs
index 84619c0..e21d7b0 100644
--- a/rayon-demo/src/main.rs
+++ b/rayon-demo/src/main.rs
@@ -1,5 +1,9 @@
 #![cfg_attr(test, feature(test))]

+extern crate jemallocator;
+#[global_allocator]
+static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
+
 use std::env;
 use std::io;
 use std::io::prelude::*;
