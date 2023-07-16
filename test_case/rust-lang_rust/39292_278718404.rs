
diff --git a/src/librustc/traits/mod.rs b/src/librustc/traits/mod.rs
index 4893e24091..71d67b106a 100644
--- a/src/librustc/traits/mod.rs
+++ b/src/librustc/traits/mod.rs
@@ -628,6 +628,7 @@ pub fn get_vtable_methods<'a, 'tcx>(
                                           |_, _| tcx.mk_region(ty::ReErased),
                                           |def, _| trait_ref.substs().type_for_def(def));
 
+            let substs = tcx.erase_late_bound_regions_and_normalize(&ty::Binder(substs));
             // It's possible that the method relies on where clauses that
             // do not hold for this particular set of type parameters.
             // Note that this method could then never be called, so we
