
diff --git a/src/librustc/middle/typeck/check/method.rs b/src/librustc/middle/typeck/check/method.rs
index 48d630b..36c9b30 100644
--- a/src/librustc/middle/typeck/check/method.rs
+++ b/src/librustc/middle/typeck/check/method.rs
@@ -372,7 +372,7 @@ impl<'self> LookupContext<'self> {
     // to a trait and its supertraits.
     fn get_method_index(&self,
                         trait_ref: @TraitRef,
-                        subtrait_id: ast::DefId,
+                                               subtrait: @TraitRef,
                         n_method: uint) -> uint {
         let tcx = self.tcx();

@@ -382,15 +382,14 @@ impl<'self> LookupContext<'self> {
         // we find the trait the method came from, counting up the
         // methods from them.
         let mut method_count = 0;
-        do ty::each_bound_trait_and_supertraits(tcx, &[trait_ref])
+        do ty::each_bound_trait_and_supertraits(tcx, &[subtrait])
             |bound_ref| {
-            if bound_ref.def_id == subtrait_id { false }
+            if bound_ref.def_id == trait_ref.def_id { false }
                 else {
-                method_count += ty::trait_methods(tcx, bound_ref.def_id).len();
+                method_count+=ty::trait_methods(tcx, bound_ref.def_id).len();
                 true
             }
         };
-
         return method_count + n_method;
     }

@@ -418,9 +417,9 @@ impl<'self> LookupContext<'self> {
         let trait_ref = @TraitRef { def_id: did, substs: rcvr_substs.clone() };

         do self.push_inherent_candidates_from_bounds_inner(&[trait_ref])
-            |trait_ref, m, method_num, _bound_num| {
+            |new_trait_ref, m, method_num, _bound_num| {
             let vtable_index =
-                self.get_method_index(trait_ref, trait_ref.def_id, method_num);
+                self.get_method_index(new_trait_ref, trait_ref, method_num);
             // We need to fix up the transformed self type.
             let transformed_self_ty =
                 self.construct_transformed_self_ty_for_object(
