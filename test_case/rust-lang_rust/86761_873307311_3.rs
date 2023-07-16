diff
--- a/compiler/rustc_mir_build/src/thir/pattern/check_match.rs
+++ b/compiler/rustc_mir_build/src/thir/pattern/check_match.rs
@@ -84,7 +84,7 @@ fn visit_param(&mut self, param: &'tcx hir::Param<'tcx>) {
 }
 
 impl PatCtxt<'_, '_> {
-    fn report_inlining_errors(&self, pat_span: Span) {
+    fn report_inlining_errors(&self) {
         for error in &self.errors {
             match *error {
                 PatternError::StaticInPattern(span) => {
@@ -96,14 +96,6 @@ fn report_inlining_errors(&self, pat_span: Span) {
                 PatternError::ConstParamInPattern(span) => {
                     self.span_e0158(span, "const parameters cannot be referenced in patterns")
                 }
-                PatternError::FloatBug => {
-                    // FIXME(#31407) this is only necessary because float parsing is buggy
-                    rustc_middle::mir::interpret::struct_error(
-                        self.tcx.at(pat_span),
-                        "could not evaluate float literal (see issue #31407)",
-                    )
-                    .emit();
-                }
--- a/compiler/rustc_mir_build/src/thir/pattern/mod.rs
+++ b/compiler/rustc_mir_build/src/thir/pattern/mod.rs
@@ -31,7 +31,6 @@
     AssocConstInPattern(Span),
     ConstParamInPattern(Span),
     StaticInPattern(Span),
-    FloatBug,
     NonConstPath(Span),
 }
