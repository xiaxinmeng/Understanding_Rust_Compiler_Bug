diff
diff --git a/src/tools/rustc-workspace-hack/Cargo.toml b/src/tools/rustc-workspace-hack/Cargo.toml
index 351e2d4481c..11d61606ff5 100644
--- a/src/tools/rustc-workspace-hack/Cargo.toml
+++ b/src/tools/rustc-workspace-hack/Cargo.toml
@@ -69,7 +69,7 @@ serde = { version = "1.0.82", features = ['derive'] }
 serde_json = { version = "1.0.31", features = ["raw_value"] }
 smallvec-0_6 = { package = "smallvec", version = "0.6", features = ['union', 'may_dangle'] }
 smallvec = { version = "1.0", features = ['union', 'may_dangle'] }
-syn = { version = "1", features = ['fold', 'full', 'extra-traits', 'visit'] }
+syn = { version = "1", features = ['fold', 'full', 'extra-traits', 'visit', 'visit-mut'] }
 url = { version = "2.0", features = ['serde'] }
 
 [target.'cfg(not(windows))'.dependencies]
