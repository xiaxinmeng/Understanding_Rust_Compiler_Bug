diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 6ba1b1b6036..d389209150f 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -898,11 +898,7 @@ pub fn cargo(
             Color::Auto => {} // nothing to do
         }

-        if cmd != "install" {
-            cargo.arg("--target").arg(target.rustc_target_arg());
-        } else {
-            assert_eq!(target, compiler.host);
-        }
+        assert_eq!(target, compiler.host);

         // Set a flag for `check`/`clippy`/`fix`, so that certain build
         // scripts can do less work (i.e. not building/requiring LLVM).
diff --git a/src/bootstrap/compile.rs b/src/bootstrap/compile.rs
index 007ca9f7f5a..a6859b297aa 100644
--- a/src/bootstrap/compile.rs
+++ b/src/bootstrap/compile.rs
@@ -1225,12 +1225,7 @@ pub fn run_cargo(
     // `target_deps_dir` looks like $dir/$target/release/deps
     let target_deps_dir = target_root_dir.join("deps");
     // `host_root_dir` looks like $dir/release
-    let host_root_dir = target_root_dir
-        .parent()
-        .unwrap() // chop off `release`
-        .parent()
-        .unwrap() // chop off `$target`
-        .join(target_root_dir.file_name().unwrap());
+    let host_root_dir = target_root_dir.clone();

     // Spawn Cargo slurping up its JSON output. We'll start building up the
     // `deps` array of all files it generated along with a `toplevel` array of
diff --git a/src/bootstrap/lib.rs b/src/bootstrap/lib.rs
index 1667dfc3f85..04a7036cf4b 100644
--- a/src/bootstrap/lib.rs
+++ b/src/bootstrap/lib.rs
@@ -729,8 +733,8 @@ fn stage_out(&self, compiler: Compiler, mode: Mode) -> PathBuf {
     /// Returns the root output directory for all Cargo output in a given stage,
     /// running a particular compiler, whether or not we're building the
     /// standard library, and targeting the specified architecture.
-    fn cargo_out(&self, compiler: Compiler, mode: Mode, target: TargetSelection) -> PathBuf {
-        self.stage_out(compiler, mode).join(&*target.triple).join(self.cargo_dir())
+    fn cargo_out(&self, compiler: Compiler, mode: Mode, _target: TargetSelection) -> PathBuf {
+        self.stage_out(compiler, mode).join(self.cargo_dir())
     }

     /// Root output directory for LLVM compiled for `target`
