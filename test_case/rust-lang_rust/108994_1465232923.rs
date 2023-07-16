diff
diff --git a/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs b/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs
index 704b0d0bd1c..541cd4981e8 100644
--- a/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs
+++ b/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs
@@ -2101,6 +2101,10 @@ fn report_similar_impl_candidates(
                     }
                     // Avoid mentioning types that are private to another crate
                     else if let ty::Adt(def, _) = self_ty.peel_refs().kind() {
+                        // Avoid mentioning unstable types on stable
+                        if !self.tcx.sess.opts.unstable_features.is_nightly_build() && self.tcx.has_attr(def.did(), sym::unstable) {
+                            return false;
+                        }
                         // FIXME(compiler-errors): This could be generalized, both to
                         // be more granular, and probably look past other `#[fundamental]`
                         // types, too.
