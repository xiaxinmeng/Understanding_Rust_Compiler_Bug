
diff --git a/src/config.rs b/src/config.rs
index 7a988a5..cfbbd93 100644
--- a/src/config.rs
+++ b/src/config.rs
@@ -129,6 +129,8 @@ pub struct Config {
     pub features: Vec<String>,
     pub all_features: bool,
     pub no_default_features: bool,
+    pub pub_only: bool,
+    pub reachable_only: bool,
 }
 
 impl Default for Config {
@@ -153,6 +155,8 @@ impl Default for Config {
             features: vec![],
             all_features: false,
             no_default_features: false,
+            pub_only: false,
+            reachable_only: false
         };
         result.normalise();
         result
