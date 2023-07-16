diff
diff --git a/src/tools/rustc-workspace-hack/Cargo.toml b/src/tools/rustc-workspace-hack/Cargo.toml
index 930279c0ca..bafddee99f 100644
--- a/src/tools/rustc-workspace-hack/Cargo.toml
+++ b/src/tools/rustc-workspace-hack/Cargo.toml
@@ -62,7 +62,7 @@ crossbeam-utils = { version = "0.6.5", features = ["nightly"] }
 serde = { version = "1.0.82", features = ['derive'] }
 serde_json = { version = "1.0.31", features = ["raw_value"] }
 smallvec = { version = "0.6", features = ['union', 'may_dangle'] }
-
+url = { version = "2.0", features = ['serde'] }

 [target.'cfg(not(windows))'.dependencies]
 openssl = { version = "0.10.12", optional = true }
