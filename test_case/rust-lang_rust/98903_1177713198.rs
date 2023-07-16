diff
diff --git a/examples/Cargo.toml b/examples/Cargo.toml
index 4528a5c..82c6b30 100644
--- a/examples/Cargo.toml
+++ b/examples/Cargo.toml
@@ -1,4 +1,5 @@
 [workspace]
+resolver = "2"
 members = [
     "function",
     "cb",
diff --git a/examples/array/Cargo.toml b/examples/array/Cargo.toml
index b77b02b..8bac645 100644
--- a/examples/array/Cargo.toml
+++ b/examples/array/Cargo.toml
@@ -14,4 +14,4 @@ node-bindgen = { path = "../.."}


 [build-dependencies]
-node-bindgen = { path = "../../", features = ["build"] }
+node-bindgen = { path = "../../", default-features = false, features = ["build"] }
