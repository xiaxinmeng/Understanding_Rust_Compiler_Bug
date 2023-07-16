 diff
diff --git a/src/librustc/session/config.rs b/src/librustc/session/config.rs
index b999929..dc8f9fe 100644
--- a/src/librustc/session/config.rs
+++ b/src/librustc/session/config.rs
@@ -849,6 +849,9 @@ pub fn rustc_optgroups() -> Vec<RustcOptGroup> {
 // Convert strings provided as --cfg [cfgspec] into a crate_cfg
 pub fn parse_cfgspecs(cfgspecs: Vec<String> ) -> ast::CrateConfig {
     cfgspecs.into_iter().map(|s| {
+        if &s == "" {
+            early_error("empty --cfg argument");
+        }
         parse::parse_meta_from_source_str("cfgspec".to_string(),
                                           s.to_string(),
                                           Vec::new(),
