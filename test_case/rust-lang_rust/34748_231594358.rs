 patch
diff --git a/Cargo.toml b/Cargo.toml
index 118e1a7..f829c1e 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14 +14 @@ keywords = ["docker"]
-hyper = "^0.7"
+hyper = "^0.9"
@@ -17 +17 @@ unix_socket = "^0.5"
-rustc-serialize = "0.3.16"
+rustc-serialize = "0.3.19"
diff --git a/src/docker.rs b/src/docker.rs
index 8f50d8f..2ae8672 100644
--- a/src/docker.rs
+++ b/src/docker.rs
@@ -78 +78 @@ impl Docker {
-    fn get_url(&mut self, path: String) -> String {
+    fn get_url(&self, path: String) -> String {
@@ -249 +249 @@ impl Docker {
-    pub fn get_images(&mut self, all: bool) -> std::io::Result<Vec<Image>> {
+    pub fn get_images(&self, all: bool) -> std::io::Result<Vec<Image>> {
