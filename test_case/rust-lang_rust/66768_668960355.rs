diff
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -4,8 +4,8 @@ version = "0.1.0"
 edition = "2018"

 [dependencies.nalgebra]
-git = "https://github.com/rustsim/nalgebra.git"
-rev = "31ef5f0ab02c6ecf279867f07cd63e16cece8b75"
+git = "https://github.com/daingun/nalgebra.git"
+branch = "patch-2"
 default-features = false
 features = ["serde-serialize", "default"]
