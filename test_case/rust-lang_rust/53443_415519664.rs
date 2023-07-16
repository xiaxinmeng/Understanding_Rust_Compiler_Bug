diff
--- a/src/librustc/middle/resolve_lifetime.rs
+++ b/src/librustc/middle/resolve_lifetime.rs
@@ -623,7 +623,13 @@ impl<'a, 'tcx> Visitor<'tcx> for LifetimeContext<'a, 'tcx> {
             }
             hir::TyKind::Path(hir::QPath::Resolved(None, ref path)) => {
                 if let Def::Existential(exist_ty_did) = path.def {
-                    let id = self.tcx.hir.as_local_node_id(exist_ty_did).unwrap();
+                    let id = match self.tcx.hir.as_local_node_id(exist_ty_did) {
+                        Some(id) => id,
+                        None => {
+                            intravisit::walk_ty(self, ty);
+                            return;
+                        },
+                    };

                     // Resolve the lifetimes in the bounds to the lifetime defs in the generics.
                     // `fn foo<'a>() -> impl MyTrait<'a> { ... }` desugars to
