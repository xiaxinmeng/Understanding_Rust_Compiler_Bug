 patch
diff --git a/src/librustc/traits/error_reporting.rs b/src/librustc/traits/error_reporting.rs
index d7ddfc9..b70a813 100644
--- a/src/librustc/traits/error_reporting.rs
+++ b/src/librustc/traits/error_reporting.rs
@@ -861,10 +861,26 @@ fn note_obligation_cause_code<'a, 'tcx, T>(infcx: &InferCtxt<'a, 'tcx>,
         }
         ObligationCauseCode::BuiltinDerivedObligation(ref data) => {
             let parent_trait_ref = infcx.resolve_type_vars_if_possible(&data.parent_trait_ref);
+            let ty = parent_trait_ref.0.self_ty();
             err.fileline_note(
                 cause_span,
                 &format!("required because it appears within the type `{}`",
-                         parent_trait_ref.0.self_ty()));
+                         ty));
+
+            // In case the requirement is that a closure be Send,
+            if Some(parent_trait_ref.0.def_id) == infcx.tcx.lang_items.send_trait() {
+                if let ty::TyClosure(did, _) = ty.sty {
+                    if let Some(span) = infcx.tcx.map.span_if_local(did) {
+                        let suggestion = match infcx.tcx.sess.codemap().span_to_snippet(span) {
+                            Ok(string) => format!("move {}", string),
+                            Err(_) => format!("move |<args>| <body>")
+                        };
+                        err.span_suggestion(span,
+                                            "Perhaps try marking this closure as a `move` closure?",
+                                            suggestion);
+                    }
+                }
+            }
             let parent_predicate = parent_trait_ref.to_predicate();
             note_obligation_cause_code(infcx,
                                        err,
