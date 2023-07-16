diff
diff --git a/src/tools/clippy/clippy_dev/src/new_lint.rs b/src/tools/clippy/clippy_dev/src/new_lint.rs
index d951ca0e630..4676c2affad 100644
--- a/src/tools/clippy/clippy_dev/src/new_lint.rs
+++ b/src/tools/clippy/clippy_dev/src/new_lint.rs
@@ -44,7 +44,7 @@ pub fn create(pass: Option<&str>, lint_name: Option<&str>, category: Option<&str
     create_test(&lint).context("Unable to create a test for the new lint")
 }
 
-fn create_lint(lint: &LintData) -> io::Result<()> {
+fn create_lint(lint: &LintData<'_>) -> io::Result<()> {
     let (pass_type, pass_lifetimes, pass_import, context_import) = match lint.pass {
         "early" => ("EarlyLintPass", "", "use rustc_ast::ast::*;", "EarlyContext"),
         "late" => ("LateLintPass", "<'_>", "use rustc_hir::*;", "LateContext"),
@@ -68,7 +68,7 @@ fn create_lint(lint: &LintData) -> io::Result<()> {
     write_file(lint.project_root.join(&lint_path), lint_contents.as_bytes())
 }
 
-fn create_test(lint: &LintData) -> io::Result<()> {
+fn create_test(lint: &LintData<'_>) -> io::Result<()> {
     fn create_project_layout<P: Into<PathBuf>>(lint_name: &str, location: P, case: &str, hint: &str) -> io::Result<()> {
         let mut path = location.into().join(case);
         fs::create_dir(&path)?;
