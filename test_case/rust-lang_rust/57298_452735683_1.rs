patch
diff --git a/clippy_lints/src/lib.rs b/clippy_lints/src/lib.rs
index 35c00fb6..4a5972f8 100644
--- a/clippy_lints/src/lib.rs
+++ b/clippy_lints/src/lib.rs
@@ -180,6 +180,7 @@ pub mod precedence;
 pub mod ptr;
 pub mod ptr_offset_with_cast;
 pub mod question_mark;
+pub mod random_state;
 pub mod ranges;
 pub mod redundant_clone;
 pub mod redundant_field_names;
@@ -486,6 +487,7 @@ pub fn register_plugins(reg: &mut rustc_plugin::Registry<'_>, conf: &Conf) {
     reg.register_late_lint_pass(box ptr_offset_with_cast::Pass);
     reg.register_late_lint_pass(box redundant_clone::RedundantClone);
     reg.register_late_lint_pass(box slow_vector_initialization::Pass);
+    reg.register_late_lint_pass(box random_state::Pass);
     reg.register_late_lint_pass(box types::RefToMut);
 
     reg.register_lint_group("clippy::restriction", Some("clippy_restriction"), vec![
@@ -1028,6 +1030,7 @@ pub fn register_plugins(reg: &mut rustc_plugin::Registry<'_>, conf: &Conf) {
         fallible_impl_from::FALLIBLE_IMPL_FROM,
         mutex_atomic::MUTEX_INTEGER,
         needless_borrow::NEEDLESS_BORROW,
+        random_state::RANDOM_STATE,
         redundant_clone::REDUNDANT_CLONE,
         unwrap::PANICKING_UNWRAP,
         unwrap::UNNECESSARY_UNWRAP,
diff --git a/clippy_lints/src/random_state.rs b/clippy_lints/src/random_state.rs
new file mode 100644
index 00000000..930733d7
--- /dev/null
+++ b/clippy_lints/src/random_state.rs
@@ -0,0 +1,51 @@
+use crate::utils::{match_type, paths, span_lint};
+use rustc::hir::Ty;
+use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
+use rustc::ty::subst::UnpackedKind;
+use rustc::ty::TyKind;
+use rustc::{declare_tool_lint, lint_array};
+
+/// **What it does:** Checks for usage of `RandomState`
+///
+/// **Why is this bad?** Some applications don't need collision prevention
+/// which lowers the performance.
+///
+/// **Known problems:** None.
+///
+/// **Example:**
+/// 