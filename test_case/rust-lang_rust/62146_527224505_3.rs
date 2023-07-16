diff
diff --git a/le-proc-macro/Cargo.toml b/le-proc-macro/Cargo.toml
index 5e905ab..bc6def3 100644
--- a/le-proc-macro/Cargo.toml
+++ b/le-proc-macro/Cargo.toml
@@ -7,7 +7,7 @@ edition = "2018"
 # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 
 [dependencies]
-nom = "5.0.0"
+nom = { path = "../nom", version = "5.0.0" }
