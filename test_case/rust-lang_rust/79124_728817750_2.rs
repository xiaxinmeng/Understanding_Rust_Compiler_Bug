diff
diff --git a/src/bootstrap/config.rs b/src/bootstrap/config.rs
index 94319a6d1e9..eb0e78bc941 100644
--- a/src/bootstrap/config.rs
+++ b/src/bootstrap/config.rs
@@ -341,7 +341,7 @@ fn do_merge<T: Merge>(x: &mut Option<T>, y: Option<T>) {
 }

 /// TOML representation of various global build decisions.
-#[derive(Deserialize, Default, Clone, Merge)]
+#[derive(Deserialize, Default, Clone, Merge, Debug)]
 #[serde(deny_unknown_fields, rename_all = "kebab-case")]
 struct Build {
     build: Option<String>,
@@ -621,8 +621,8 @@ pub fn parse(args: &[String]) -> Config {
             config.config = cfg;
         }

+        println!("Build: {:?}", toml.build);
         let build = toml.build.unwrap_or_default();
-
         config.hosts = if let Some(arg_host) = flags.host {
             arg_host
         } else if let Some(file_host) = build.host {
@@ -899,7 +899,14 @@ pub fn parse(args: &[String]) -> Config {
                 target.sanitizers = cfg.sanitizers.unwrap_or(build.sanitizers.unwrap_or_default());
                 target.profiler = cfg.profiler.unwrap_or(build.profiler.unwrap_or_default());

+                println!("`sanitizers` in [build]: {:?}", build.sanitizers);
+                println!("`profiler` in [build]: {:?}", build.profiler);
+                println!("`sanitizers` in [target.{:?}]: {:?}", triple, cfg.sanitizers);
+                println!("`profiler` in [target.{:?}]: {:?}", triple, cfg.profiler);
+                println!("final sanitizers config for {:?} : {:?}", triple, target.sanitizers);
+                println!("final profiler config for {:?} profiler: {:?}", triple, target.profiler);
                 config.target_config.insert(TargetSelection::from_user(&triple), target);
+
             }
         }

