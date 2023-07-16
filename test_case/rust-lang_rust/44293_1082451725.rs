diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index e7511888114..a9c2d38b644 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -431,7 +431,7 @@ fn parse(string: &str) -> Kind {
         }
     }
 
-    fn as_str(&self) -> &'static str {
+    pub(crate) fn as_str(&self) -> &'static str {
         match self {
             Kind::Build => "build",
             Kind::Check => "check",
@@ -456,6 +456,8 @@ fn get_step_descriptions(kind: Kind) -> Vec<StepDescription> {
         }
         match kind {
             Kind::Build => describe!(
+                compile::Crate,
+                compile::CrateLibrustc,
                 compile::Std,
                 compile::Assemble,
                 compile::CodegenBackend,
diff --git a/src/bootstrap/compile.rs b/src/bootstrap/compile.rs
index 00fc1f04342..aa6429e6b80 100644
--- a/src/bootstrap/compile.rs
+++ b/src/bootstrap/compile.rs
@@ -517,6 +517,115 @@ fn run(self, builder: &Builder<'_>) -> Vec<(PathBuf, DependencyType)> {
     }
 }
 
+#[derive(Debug, PartialOrd, Ord, Copy, Clone, PartialEq, Eq, Hash)]
+pub struct CrateLibrustc {}
+impl Step for CrateLibrustc {
+    type Output = ();
+    const DEFAULT: bool = true;
+    const ONLY_HOSTS: bool = true;
+
+    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
+        run.krate("rustc-main")
+    }
+
+    fn make_run(run: RunConfig<'_>) {
+        let builder = run.builder;
+        let host = builder.config.build;
+        let compiler = builder.compiler_for(builder.top_stage, host, host);
+
+        for krate in builder.in_tree_crates("rustc-main", Some(run.target)) {
+            if krate.path.ends_with(&run.path) {
+                builder.ensure(Crate {
+                    compiler,
+                    target: run.target,
+                    mode: Mode::Rustc,
+                    krate: krate.name,
+                });
+            }
+        }
+    }
+
+    fn run(self, _: &Builder<'_>) {
+        unreachable!()
+    }
+}
+
+/// Build a single crate using cargo.
+///
+/// FIXME: once test flags are unified under `builder.cargo`,
+/// unify this with `test::Crate`.
+#[derive(Debug, PartialOrd, Ord, Copy, Clone, PartialEq, Eq, Hash)]
+pub struct Crate {
+    pub target: TargetSelection,
+    pub compiler: Compiler,
+    pub mode: Mode,
+    pub krate: Interned<String>,
+}
+
+impl Step for Crate {
+    type Output = ();
+    const DEFAULT: bool = true;
+    const ONLY_HOSTS: bool = true;
+
+    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
+        run.krate("test")
+    }
+
+    fn make_run(run: RunConfig<'_>) {
+        let builder = run.builder;
+        let host = builder.config.build;
+        let compiler = builder.compiler_for(builder.top_stage, host, host);
+
+        for krate in builder.in_tree_crates("test", Some(run.target)) {
+            if krate.path.ends_with(&run.path) {
+                builder.ensure(Crate {
+                    compiler,
+                    target: run.target,
+                    // NOTE: ok to hard-code `Std`, even though `CrateRustc` uses `ensure(Crate)`,
+                    // because `make_run` is only called for dependencies of `test`.
+                    mode: Mode::Std,
+                    krate: krate.name,
+                });
+            }
+        }
+    }
+
+    fn run(self, builder: &Builder<'_>) {
+        let Crate { compiler, mode, target, krate } = self;
+
+        let mut cargo =
+            builder.cargo(compiler, mode, SourceType::InTree, target, builder.kind.as_str());
+        let stamp = match mode {
+            Mode::Std => {
+                std_cargo(builder, target, compiler.stage, &mut cargo);
+                libstd_stamp(builder, compiler, target)
+            }
+            Mode::Rustc => {
+                builder.ensure(Std { compiler, target });
+                rustc_cargo(builder, &mut cargo, target);
+                librustc_stamp(builder, compiler, target)
+            }
+            _ => panic!("crate-specific builds only supported for library and compiler"),
+        };
+
+        cargo.arg("-p").arg(krate);
+
+        builder.info(&format!(
+            "{} {} stage{} ({} -> {})",
+            builder.kind.as_str(), krate, compiler.stage, &compiler.host, target
+        ));
+
+        run_cargo(
+            builder,
+            cargo,
+            vec![],
+            &stamp,
+            vec![],
+            false,
+        );
+    }
+}
+
