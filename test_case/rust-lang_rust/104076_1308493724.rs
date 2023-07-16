
diff --git a/src/bootstrap/compile.rs b/src/bootstrap/compile.rs
index f0649577984..39ccefc0bc0 100644
--- a/src/bootstrap/compile.rs
+++ b/src/bootstrap/compile.rs
@@ -1122,13 +1122,18 @@ fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
     fn run(self, builder: &Builder<'_>) -> Interned<PathBuf> {
         let compiler = self.compiler;
         let host_dir = builder.out.join(&compiler.host.triple);
-        let sysroot = if compiler.stage == 0 {
-            host_dir.join("stage0-sysroot")
-        } else if builder.download_rustc() && compiler.stage != builder.top_stage {
-            host_dir.join("ci-rustc-sysroot")
-        } else {
-            host_dir.join(format!("stage{}", compiler.stage))
+
+        let sysroot_dir = |stage| {
+            if stage == 0 {
+                host_dir.join("stage0-sysroot")
+            } else if builder.download_rustc() {
+                host_dir.join("ci-rustc-sysroot")
+            } else {
+                host_dir.join(format!("stage{}", stage))
+            }
         };
+
+        let sysroot = dbg!(sysroot_dir(compiler.stage));
         let _ = fs::remove_dir_all(&sysroot);
         t!(fs::create_dir_all(&sysroot));
 
@@ -1140,8 +1145,15 @@ fn run(self, builder: &Builder<'_>) -> Interned<PathBuf> {
             );
 
             // #102002, cleanup stage1 and stage0-sysroot folders when using download-rustc so people don't use old versions of the toolchain by accident.
-            let _ = fs::remove_dir_all(host_dir.join("stage1"));
-            let _ = fs::remove_dir_all(host_dir.join("stage0-sysroot"));
+            for stage in 0..=2 {
+                if stage != compiler.stage {
+                    let dir = sysroot_dir(stage);
+                    if !dir.ends_with("ci-rustc-sysroot") {
+                        println!("removing {}", dir.display());
+                        let _ = fs::remove_dir_all(dir);
+                    }
+                }
+            }
 
             // Copy the compiler into the correct sysroot.
             let ci_rustc_dir =
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index d3952206947..96a59d7b086 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -518,8 +518,6 @@ fn run(self, builder: &Builder<'_>) -> PathBuf {
         // When using `download-rustc` and a stage0 build_compiler, copying rustc doesn't actually
         // build stage0 libstd (because the libstd in sysroot has the wrong ABI). Explicitly build
         // it.
-        builder.ensure(compile::Std::new(build_compiler, target_compiler.host));
-        builder.ensure(compile::Rustc::new(build_compiler, target_compiler.host));
         // NOTE: this implies that `download-rustc` is pretty useless when compiling with the stage0
         // compiler, since you do just as much work.
         if !builder.config.dry_run && builder.download_rustc() && build_compiler.stage == 0 {
@@ -527,6 +525,8 @@ fn run(self, builder: &Builder<'_>) -> PathBuf {
                 "warning: `download-rustc` does nothing when building stage1 tools; consider using `--stage 2` instead"
             );
         }
+        builder.ensure(compile::Std::new(build_compiler, target_compiler.host));
+        builder.ensure(compile::Rustc::new(build_compiler, target_compiler.host));
 
         // The presence of `target_compiler` ensures that the necessary libraries (codegen backends,
         // compiler libraries, ...) are built. Rustdoc does not require the presence of any
diff --git a/src/librustdoc/config.rs b/src/librustdoc/config.rs
index 9c08eac4edc..2016960edd7 100644
--- a/src/librustdoc/config.rs
+++ b/src/librustdoc/config.rs
@@ -326,7 +326,7 @@ pub(crate) fn from_matches(
             crate::usage("rustdoc");
             return Err(0);
         } else if matches.opt_present("version") {
-            rustc_driver::version("rustdoc", matches);
+            println!("local");
             return Err(0);
         }
 