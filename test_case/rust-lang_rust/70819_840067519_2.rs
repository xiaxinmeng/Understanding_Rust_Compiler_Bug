diff
diff --git a/compiler/rustc_session/src/config.rs b/compiler/rustc_session/src/config.rs
index f517c483758..436afad11b8 100644
--- a/compiler/rustc_session/src/config.rs
+++ b/compiler/rustc_session/src/config.rs
@@ -1161,15 +1161,7 @@ pub fn get_cmd_lint_options(
     let mut describe_lints = false;
 
     for &level in &[lint::Allow, lint::Warn, lint::Deny, lint::Forbid] {
-        for (passed_arg_pos, lint_name) in matches.opt_strs_pos(level.as_str()) {
-            let arg_pos = if let lint::Forbid = level {
-                // HACK: forbid is always specified last, so it can't be overridden.
-                // FIXME: remove this once <https://github.com/rust-lang/rust/issues/70819> is
-                // fixed and `forbid` works as expected.
-                usize::MAX
-            } else {
-                passed_arg_pos
-            };
+        for (arg_pos, lint_name) in matches.opt_strs_pos(level.as_str()) {
             if lint_name == "help" {
                 describe_lints = true;
             } else {
