patch
diff --git a/src/cargo/core/compiler/build_config.rs b/src/cargo/core/compiler/build_config.rs
index 4b488261c..cbe308dab 100644
--- a/src/cargo/core/compiler/build_config.rs
+++ b/src/cargo/core/compiler/build_config.rs
@@ -147,7 +147,7 @@ pub enum CompileMode {
     /// A target that will be tested with `rustdoc`.
     Doctest,
     /// A marker for Units that represent the execution of a `build.rs` script.
-    RunCustomBuild,
+    RunCustomBuild { parent_is_check: bool, },
 }
 
 impl ser::Serialize for CompileMode {
@@ -163,7 +163,7 @@ impl ser::Serialize for CompileMode {
             Bench => "bench".serialize(s),
             Doc { .. } => "doc".serialize(s),
             Doctest => "doctest".serialize(s),
-            RunCustomBuild => "run-custom-build".serialize(s),
+            RunCustomBuild { .. } => "run-custom-build".serialize(s),
         }
     }
 }
@@ -174,6 +174,11 @@ impl CompileMode {
         matches!(self, CompileMode::Check { .. })
     }
 
+    /// Returns `true` if the unit or any of its parents is being checked.
+    pub fn is_any_parent_unit_check(self) -> bool {
+        matches!(self, CompileMode::Check { .. } | CompileMode::RunCustomBuild { parent_is_check: true })
+    }
+
     /// Returns `true` if this is generating documentation.
     pub fn is_doc(self) -> bool {
         matches!(self, CompileMode::Doc { .. })
@@ -206,6 +211,6 @@ impl CompileMode {
 
     /// Returns `true` if this is the *execution* of a `build.rs` script.
     pub fn is_run_custom_build(self) -> bool {
-        self == CompileMode::RunCustomBuild
+        matches!(self, CompileMode::RunCustomBuild { .. })
     }
 }
diff --git a/src/cargo/core/compiler/build_context/target_info.rs b/src/cargo/core/compiler/build_context/target_info.rs
index 610749af9..5f729132c 100644
--- a/src/cargo/core/compiler/build_context/target_info.rs
+++ b/src/cargo/core/compiler/build_context/target_info.rs
@@ -445,7 +445,7 @@ impl TargetInfo {
                 }
             }
             CompileMode::Check { .. } => Ok((vec![FileType::new_rmeta()], Vec::new())),
-            CompileMode::Doc { .. } | CompileMode::Doctest | CompileMode::RunCustomBuild => {
+            CompileMode::Doc { .. } | CompileMode::Doctest | CompileMode::RunCustomBuild { .. } => {
                 panic!("asked for rustc output for non-rustc mode")
             }
         }
diff --git a/src/cargo/core/compiler/compilation.rs b/src/cargo/core/compiler/compilation.rs
index c1f5abce3..8589c01b2 100644
--- a/src/cargo/core/compiler/compilation.rs
+++ b/src/cargo/core/compiler/compilation.rs
@@ -176,7 +176,7 @@ impl<'cfg> Compilation<'cfg> {
         };
 
         let cmd = fill_rustc_tool_env(rustc, unit);
-        self.fill_env(cmd, &unit.pkg, None, unit.kind, true)
+        self.fill_env(cmd, unit,  &unit.pkg, None, unit.kind, true)
     }
 
     /// Returns a [`ProcessBuilder`] for running `rustdoc`.
@@ -187,7 +187,7 @@ impl<'cfg> Compilation<'cfg> {
     ) -> CargoResult<ProcessBuilder> {
         let rustdoc = ProcessBuilder::new(&*self.config.rustdoc()?);
         let cmd = fill_rustc_tool_env(rustdoc, unit);
-        let mut p = self.fill_env(cmd, &unit.pkg, script_meta, unit.kind, true)?;
+        let mut p = self.fill_env(cmd, unit,  &unit.pkg, script_meta, unit.kind, true)?;
         unit.target.edition().cmd_edition_arg(&mut p);
 
         for crate_type in unit.target.rustc_crate_types() {
@@ -206,10 +206,12 @@ impl<'cfg> Compilation<'cfg> {
     pub fn host_process<T: AsRef<OsStr>>(
         &self,
         cmd: T,
+        unit: &Unit,
         pkg: &Package,
     ) -> CargoResult<ProcessBuilder> {
         self.fill_env(
             ProcessBuilder::new(cmd),
+            unit,
             pkg,
             None,
             CompileKind::Host,
@@ -232,6 +234,7 @@ impl<'cfg> Compilation<'cfg> {
         &self,
         cmd: T,
         kind: CompileKind,
+        unit: &Unit,
         pkg: &Package,
         script_meta: Option<Metadata>,
     ) -> CargoResult<ProcessBuilder> {
@@ -243,7 +246,7 @@ impl<'cfg> Compilation<'cfg> {
         } else {
             ProcessBuilder::new(cmd)
         };
-        self.fill_env(builder, pkg, script_meta, kind, false)
+        self.fill_env(builder, unit, pkg, script_meta, kind, false)
     }
 
     /// Prepares a new process with an appropriate environment to run against
@@ -254,6 +257,7 @@ impl<'cfg> Compilation<'cfg> {
     fn fill_env(
         &self,
         mut cmd: ProcessBuilder,
+        unit: &Unit,
         pkg: &Package,
         script_meta: Option<Metadata>,
         kind: CompileKind,
@@ -354,6 +358,10 @@ impl<'cfg> Compilation<'cfg> {
             }
         }
 
+        if unit.mode.is_any_parent_unit_check() {
+            cmd.env("CARGO_CHECK", "1");
+        }
+
         Ok(cmd)
     }
 }
diff --git a/src/cargo/core/compiler/context/compilation_files.rs b/src/cargo/core/compiler/context/compilation_files.rs
index 37b11da84..1d953c77b 100644
--- a/src/cargo/core/compiler/context/compilation_files.rs
+++ b/src/cargo/core/compiler/context/compilation_files.rs
@@ -390,7 +390,7 @@ impl<'a, 'cfg: 'a> CompilationFiles<'a, 'cfg> {
                     flavor: FileFlavor::Normal,
                 }]
             }
-            CompileMode::RunCustomBuild => {
+            CompileMode::RunCustomBuild { .. } => {
                 // At this time, this code path does not handle build script
                 // outputs.
                 vec![]
diff --git a/src/cargo/core/compiler/custom_build.rs b/src/cargo/core/compiler/custom_build.rs
index 1587f100e..fb26db015 100644
--- a/src/cargo/core/compiler/custom_build.rs
+++ b/src/cargo/core/compiler/custom_build.rs
@@ -180,7 +180,7 @@ fn build_work(cx: &mut Context<'_, '_>, unit: &Unit) -> CargoResult<Job> {
     // `Profiles::get_profile_run_custom_build` so that those flags get
     // carried over.
     let to_exec = to_exec.into_os_string();
-    let mut cmd = cx.compilation.host_process(to_exec, &unit.pkg)?;
+    let mut cmd = cx.compilation.host_process(to_exec, unit, &unit.pkg)?;
     let debug = unit.profile.debuginfo.unwrap_or(0) != 0;
     cmd.env("OUT_DIR", &script_out_dir)
         .env("CARGO_MANIFEST_DIR", unit.pkg.root())
diff --git a/src/cargo/core/compiler/job_queue.rs b/src/cargo/core/compiler/job_queue.rs
index e6ce2c614..a0577b257 100644
--- a/src/cargo/core/compiler/job_queue.rs
+++ b/src/cargo/core/compiler/job_queue.rs
@@ -946,7 +946,7 @@ impl<'cfg> DrainState<'cfg> {
         let pkg_name = unit.pkg.name();
         match unit.mode {
             CompileMode::Doc { .. } => format!("{}(doc)", pkg_name),
-            CompileMode::RunCustomBuild => format!("{}(build)", pkg_name),
+            CompileMode::RunCustomBuild { .. } => format!("{}(build)", pkg_name),
             _ => {
                 let annotation = match unit.target.kind() {
                     TargetKind::Lib(_) => return pkg_name.to_string(),
diff --git a/src/cargo/core/compiler/mod.rs b/src/cargo/core/compiler/mod.rs
index 0551f7ce5..09750368e 100644
--- a/src/cargo/core/compiler/mod.rs
+++ b/src/cargo/core/compiler/mod.rs
@@ -956,6 +956,13 @@ fn build_base_args(
             cmd.env(&key, exe_path);
         }
     }
+
+    // Add `CARGO_CHECK` environment variable to notify build scripts that no final
+    // code generation will be performed
+    if unit.mode.is_any_parent_unit_check() {
+        cmd.env("CARGO_CHECK", "1");
+    }
+
     Ok(())
 }
 
diff --git a/src/cargo/core/compiler/timings.rs b/src/cargo/core/compiler/timings.rs
index 836acac5c..06f07355f 100644
--- a/src/cargo/core/compiler/timings.rs
+++ b/src/cargo/core/compiler/timings.rs
@@ -176,7 +176,7 @@ impl<'cfg> Timings<'cfg> {
             CompileMode::Bench => target.push_str(" (bench)"),
             CompileMode::Doc { .. } => target.push_str(" (doc)"),
             CompileMode::Doctest => target.push_str(" (doc test)"),
-            CompileMode::RunCustomBuild => target.push_str(" (run)"),
+            CompileMode::RunCustomBuild { .. } => target.push_str(" (run)"),
         }
         let unit_time = UnitTime {
             unit,
diff --git a/src/cargo/core/compiler/unit_dependencies.rs b/src/cargo/core/compiler/unit_dependencies.rs
index cfcda149b..3af5ac9ca 100644
--- a/src/cargo/core/compiler/unit_dependencies.rs
+++ b/src/cargo/core/compiler/unit_dependencies.rs
@@ -543,7 +543,7 @@ fn dep_build_script(
                 t,
                 script_unit_for,
                 unit.kind,
-                CompileMode::RunCustomBuild,
+                CompileMode::RunCustomBuild { parent_is_check: unit.mode.is_check() },
                 profile,
             )
         })
@@ -646,7 +646,7 @@ fn connect_run_custom_build_deps(state: &mut State<'_, '_>) {
         let mut reverse_deps_map = HashMap::new();
         for (unit, deps) in state.unit_dependencies.iter() {
             for dep in deps {
-                if dep.unit.mode == CompileMode::RunCustomBuild {
+                if dep.unit.mode.is_run_custom_build() {
                     reverse_deps_map
                         .entry(dep.unit.clone())
                         .or_insert_with(HashSet::new)
@@ -667,7 +667,7 @@ fn connect_run_custom_build_deps(state: &mut State<'_, '_>) {
         for unit in state
             .unit_dependencies
             .keys()
-            .filter(|k| k.mode == CompileMode::RunCustomBuild)
+            .filter(|k| k.mode.is_run_custom_build())
         {
             // This list of dependencies all depend on `unit`, an execution of
             // the build script.
@@ -708,7 +708,7 @@ fn connect_run_custom_build_deps(state: &mut State<'_, '_>) {
                 .filter_map(|other| {
                     state.unit_dependencies[&other.unit]
                         .iter()
-                        .find(|other_dep| other_dep.unit.mode == CompileMode::RunCustomBuild)
+                        .find(|other_dep|  other_dep.unit.mode.is_run_custom_build())
                         .cloned()
                 })
                 .collect::<HashSet<_>>();
diff --git a/src/cargo/core/profiles.rs b/src/cargo/core/profiles.rs
index a86f0899e..297f8131f 100644
--- a/src/cargo/core/profiles.rs
+++ b/src/cargo/core/profiles.rs
@@ -313,7 +313,7 @@ impl Profiles {
                         )
                     }
                 }
-                CompileMode::Build | CompileMode::Check { .. } | CompileMode::RunCustomBuild => {
+                CompileMode::Build | CompileMode::Check { .. } | CompileMode::RunCustomBuild { .. } => {
                     // Note: `RunCustomBuild` doesn't normally use this code path.
                     // `build_unit_profiles` normally ensures that it selects the
                     // ancestor's profile. However, `cargo clean -p` can hit this
diff --git a/src/cargo/ops/cargo_compile.rs b/src/cargo/ops/cargo_compile.rs
index 44945d919..89718eade 100644
--- a/src/cargo/ops/cargo_compile.rs
+++ b/src/cargo/ops/cargo_compile.rs
@@ -344,7 +344,7 @@ pub fn create_bcx<'a, 'cfg>(
         | CompileMode::Build
         | CompileMode::Check { .. }
         | CompileMode::Bench
-        | CompileMode::RunCustomBuild => {
+        | CompileMode::RunCustomBuild { .. }=> {
             if std::env::var("RUST_FLAGS").is_ok() {
                 config.shell().warn(
                     "Cargo does not read `RUST_FLAGS` environment variable. Did you mean `RUSTFLAGS`?",
@@ -747,7 +747,7 @@ impl CompileFilter {
                     } => examples.is_specific() || tests.is_specific() || benches.is_specific(),
                 }
             }
-            CompileMode::RunCustomBuild => panic!("Invalid mode"),
+            CompileMode::RunCustomBuild { .. } => panic!("Invalid mode"),
         }
     }
 
@@ -1261,7 +1261,7 @@ fn filter_default_targets(targets: &[Target], mode: CompileMode) -> Vec<&Target>
                 })
                 .collect()
         }
-        CompileMode::Doctest | CompileMode::RunCustomBuild => panic!("Invalid mode {:?}", mode),
+        CompileMode::Doctest | CompileMode::RunCustomBuild { .. } => panic!("Invalid mode {:?}", mode),
     }
 }
 
diff --git a/src/cargo/ops/cargo_run.rs b/src/cargo/ops/cargo_run.rs
index 69bae2c59..eb5229d7f 100644
--- a/src/cargo/ops/cargo_run.rs
+++ b/src/cargo/ops/cargo_run.rs
@@ -92,7 +92,7 @@ pub fn run(
         Err(_) => path.to_path_buf(),
     };
     let pkg = bins[0].0;
-    let mut process = compile.target_process(exe, unit.kind, pkg, *script_meta)?;
+    let mut process = compile.target_process(exe, unit.kind, unit, pkg, *script_meta)?;
     process.args(args).cwd(config.cwd());
 
     config.shell().status("Running", process.to_string())?;
diff --git a/src/cargo/ops/cargo_test.rs b/src/cargo/ops/cargo_test.rs
index 9fcb94f13..1232146bc 100644
--- a/src/cargo/ops/cargo_test.rs
+++ b/src/cargo/ops/cargo_test.rs
@@ -103,7 +103,7 @@ fn run_unit_tests(
             )
         };
 
-        let mut cmd = compilation.target_process(path, unit.kind, &unit.pkg, *script_meta)?;
+        let mut cmd = compilation.target_process(path, unit.kind, unit, &unit.pkg, *script_meta)?;
         cmd.args(test_args);
         if unit.target.harness() && config.shell().verbosity() == Verbosity::Quiet {
             cmd.arg("--quiet");
diff --git a/tests/testsuite/check.rs b/tests/testsuite/check.rs
index 0bfd5b923..c814c5f13 100644
--- a/tests/testsuite/check.rs
+++ b/tests/testsuite/check.rs
@@ -1001,3 +1001,55 @@ fn rustc_workspace_wrapper_excludes_published_deps() {
         .with_stdout_does_not_contain("WRAPPER CALLED: rustc --crate-name baz [..]")
         .run();
 }
+
+#[cargo_test]
+fn check_crate_env_vars() {
+    let foo = project()
+        .file(
+            "Cargo.toml",
+            r#"
+                [package]
+                name = "foo"
+                version = "0.0.1"
+                authors = []
+
+                [dependencies.bar]
+                path = "../bar"
+            "#,
+        )
+        .file(
+            "src/main.rs",
+            "extern crate bar; fn main() { ::bar::baz(); }",
+        )
+        .file(
+            "build.rs",
+            r#"
+            pub fn main() {
+                for (key, value) in std::env::vars() {
+                    println!("{}: {}", key, value);
+                }
+
+                assert!(std::env::var_os("CARGO_CHECK").is_some())
+            }"#,
+        )
+        .build();
+
+    let _bar = project()
+        .at("bar")
+        .file("Cargo.toml", &basic_manifest("bar", "0.1.0"))
+        .file("src/lib.rs", "pub fn baz() {}")
+        .file(
+            "build.rs",
+            r#"
+            pub fn main() {
+                for (key, value) in std::env::vars() {
+                    println!("{}: {}", key, value);
+                }
+                assert!(std::env::var_os("CARGO_CHECK").is_some())
+            }
+        "#,
+        )
+        .build();
+
+    foo.cargo("check").run();
+}
