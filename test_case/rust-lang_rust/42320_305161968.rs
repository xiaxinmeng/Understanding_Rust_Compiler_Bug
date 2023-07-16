diff
diff --git a/src/bootstrap/step.rs b/src/bootstrap/step.rs
--- a/src/bootstrap/step.rs
+++ b/src/bootstrap/step.rs
@@ -899,11 +899,15 @@ pub struct Rules<'a> {
 
 impl<'a> Rules<'a> {
     fn new(build: &'a Build) -> Rules<'a> {
+        let target = ::std::env::var("DEB_HOST_RUST_TYPE").unwrap();
+        // rust forces us to do this dance because of lifetimes :/
+        let hosts = &build.config.host;
+        let tidx = hosts.iter().position(|x| x == target.as_str()).unwrap();
         Rules {
             build: build,
             sbuild: Step {
                 stage: build.flags.stage.unwrap_or(2),
-                target: &build.config.build,
+                target: hosts[tidx].as_str(),
                 host: &build.config.build,
                 name: "",
             },
