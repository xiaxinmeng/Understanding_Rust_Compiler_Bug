diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 608b4c4d245..0bf7f245518 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -2202,6 +2202,17 @@ pub struct Cargo {
 }
 
 impl Cargo {
+    /// NOTE: avoid this in favor of `builder.cargo` where possible.
+    pub fn from_cmd(command: Command, target: TargetSelection) -> Cargo {
+        let rustflags = Rustflags::new(target);
+        Cargo {
+            command,
+            rustdocflags: rustflags.clone(),
+            rustflags,
+            allow_features: String::new(),
+        }
+    }
+
     pub fn rustdocflag(&mut self, arg: &str) -> &mut Cargo {
         self.rustdocflags.arg(arg);
         self
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 9540adea189..7c7e6100424 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -2035,7 +2035,7 @@ fn run(self, builder: &Builder<'_>) {
 ///
 /// Returns whether the test succeeded.
 fn run_cargo_test(
-    cargo: impl Into<Command>,
+    cargo: crate::builder::Cargo,
     libtest_args: &[&str],
     crates: &[Interned<String>],
     primary_crate: &str,
@@ -2051,7 +2051,7 @@ fn run_cargo_test(
 
 /// Given a `cargo test` subcommand, pass it the appropriate test flags given a `builder`.
 fn prepare_cargo_test(
-    cargo: impl Into<Command>,
+    mut cargo: crate::builder::Cargo,
     libtest_args: &[&str],
     crates: &[Interned<String>],
     primary_crate: &str,
@@ -2059,8 +2059,6 @@ fn prepare_cargo_test(
     target: TargetSelection,
     builder: &Builder<'_>,
 ) -> Command {
-    let mut cargo = cargo.into();
-
     // Pass in some standard flags then iterate over the graph we've discovered
     // in `cargo metadata` with the maps above and figure out what `-p`
     // arguments need to get passed.
@@ -2078,6 +2076,9 @@ fn prepare_cargo_test(
                 .unwrap_or_else(|| panic!("missing crate {primary_crate}"));
             if krate.has_lib {
                 cargo.arg("--lib");
+                if crates.iter().any(|c| c != "panic_abort") {
+                    cargo.rustflag("-Zpanic_abort_tests");
+                }
             }
             cargo.args(&["--bins", "--examples", "--tests", "--benches"]);
         }
@@ -2118,7 +2119,7 @@ fn prepare_cargo_test(
         );
     }
 
-    cargo
+    cargo.into()
 }
 
 #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
@@ -2518,7 +2519,7 @@ fn run(self, builder: &Builder<'_>) {
         }
         // rustbuild tests are racy on directory creation so just run them one at a time.
         // Since there's not many this shouldn't be a problem.
-        run_cargo_test(cmd, &["--test-threads=1"], &[], "bootstrap", compiler, host, builder);
+        run_cargo_test(crate::builder::Cargo::from_cmd(cmd, host), &["--test-threads=1"], &[], "bootstrap", compiler, host, builder);
     }
 
     fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
