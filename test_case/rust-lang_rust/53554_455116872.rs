patch
diff --git a/workspace.Cargo.toml b/workspace.Cargo.toml
index 7d53923..08313fd 100644
--- a/workspace.Cargo.toml
+++ b/workspace.Cargo.toml
@@ -1,2 +1,7 @@
 [workspace]
 members = ["macroslib", "jni_tests", "c++_tests", "android-example"]
+
+[profile.release]
+debug = true
+lto = true
