patch
diff --git a/compiler/rustc_errors/src/lib.rs b/compiler/rustc_errors/src/lib.rs
index 4d262ae0f5e..684c657620a 100644
--- a/compiler/rustc_errors/src/lib.rs
+++ b/compiler/rustc_errors/src/lib.rs
@@ -1011,6 +1011,16 @@ pub fn abort_if_errors(&self) {
         self.inner.borrow_mut().abort_if_errors()
     }
 
+    pub fn abort_if_errors_or_delayed_span_bugs(&self) {
+        self.abort_if_errors();
+        let mut inner = self.inner.borrow();
+        if inner.has_delayed_span_bugs() {
+            let bugs = inner.delayed_span_bugs.clone();
+            inner.flush_delayed(bugs, "no errors encountered even though `delay_span_bug` issued");
+            FatalError.raise();
+        }
+    }
+
     /// `true` if we haven't taught a diagnostic with this code already.
     /// The caller must then teach the user about such a diagnostic.
     ///
@@ -1414,7 +1424,10 @@ fn has_errors_or_lint_errors(&self) -> bool {
         self.has_errors() || self.lint_err_count > 0
     }
     fn has_errors_or_delayed_span_bugs(&self) -> bool {
-        self.has_errors() || !self.delayed_span_bugs.is_empty()
+        self.has_errors() || !self.has_delayed_span_bugs()
+    }
+    fn has_delayed_span_bugs(&self) -> bool {
+        !self.delayed_span_bugs.is_empty()
     }
     fn has_any_message(&self) -> bool {
         self.err_count() > 0 || self.lint_err_count > 0 || self.warn_count > 0
diff --git a/compiler/rustc_session/src/session.rs b/compiler/rustc_session/src/session.rs
index 0142e981766..0e4afbcf191 100644
--- a/compiler/rustc_session/src/session.rs
+++ b/compiler/rustc_session/src/session.rs
@@ -555,6 +555,9 @@ pub fn has_errors_or_delayed_span_bugs(&self) -> bool {
     pub fn abort_if_errors(&self) {
         self.diagnostic().abort_if_errors();
     }
+    pub fn abort_if_errors_or_delayed_span_bugs(&self) {
+        self.diagnostic().abort_if_errors_or_delayed_span_bugs();
+    }
     pub fn compile_status(&self) -> Result<(), ErrorGuaranteed> {
         if let Some(reported) = self.diagnostic().has_errors_or_lint_errors() {
             let _ = self.diagnostic().emit_stashed_diagnostics();
diff --git a/library/stdarch b/library/stdarch
--- a/library/stdarch
+++ b/library/stdarch
@@ -1 +1 @@
-Subproject commit 699c093a42283c07e9763b4c19439a900ae2d321
+Subproject commit 699c093a42283c07e9763b4c19439a900ae2d321-dirty
diff --git a/src/librustdoc/clean/mod.rs b/src/librustdoc/clean/mod.rs
index 704292c1048..a75b0c592d3 100644
--- a/src/librustdoc/clean/mod.rs
+++ b/src/librustdoc/clean/mod.rs
@@ -381,7 +381,9 @@ fn clean_hir_term<'tcx>(term: &hir::Term<'tcx>, cx: &mut DocContext<'tcx>) -> Te
         hir::Term::Ty(ty) => Term::Type(clean_ty(ty, cx)),
         hir::Term::Const(c) => {
             let def_id = cx.tcx.hir().local_def_id(c.hir_id);
-            Term::Constant(clean_middle_const(ty::Const::from_anon_const(cx.tcx, def_id), cx))
+            let ty = ty::Const::from_anon_const(cx.tcx, def_id);
+            cx.tcx.sess.abort_if_errors_or_delayed_span_bugs();
+            Term::Constant(clean_middle_const(ty, cx))
         }
     }
 }
