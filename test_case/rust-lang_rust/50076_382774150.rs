diff
diff --git a/src/librustc_mir/borrow_check/nll/type_check/mod.rs b/src/librustc_mir/borrow_check/nll/type_check/mod.rs
index acd246b703..e9c86ea99b 100644
--- a/src/librustc_mir/borrow_check/nll/type_check/mod.rs
+++ b/src/librustc_mir/borrow_check/nll/type_check/mod.rs
@@ -275,7 +275,7 @@ impl<'a, 'b, 'gcx, 'tcx> TypeVerifier<'a, 'b, 'gcx, 'tcx> {
                         tcx.predicates_of(def_id).instantiate(tcx, substs);
                     let predicates =
                         type_checker.normalize(&instantiated_predicates.predicates, location);
-                    type_checker.prove_predicates(predicates.iter().cloned(), location);
+                    type_checker.prove_predicates(predicates, location);
                 }
 
                 value.ty
@@ -1516,34 +1516,33 @@ impl<'a, 'gcx, 'tcx> TypeChecker<'a, 'gcx, 'tcx> {
 
         let predicates = self.normalize(&instantiated_predicates.predicates, location);
         debug!("prove_aggregate_predicates: predicates={:?}", predicates);
-        self.prove_predicates(predicates.iter().cloned(), location);
+        self.prove_predicates(predicates, location);
     }
 
     fn prove_trait_ref(&mut self, trait_ref: ty::TraitRef<'tcx>, location: Location) {
         self.prove_predicates(
-            [ty::Predicate::Trait(
+            Some(ty::Predicate::Trait(
                 trait_ref.to_poly_trait_ref().to_poly_trait_predicate(),
-            )].iter()
-                .cloned(),
+            )),
             location,
         );
     }
 
-    fn prove_predicates(
-        &mut self,
-        predicates: impl IntoIterator<Item = ty::Predicate<'tcx>>,
-        location: Location,
-    ) {
-        let mut predicates_iter = predicates.into_iter();
+    fn prove_predicates<T>(&mut self, predicates: T, location: Location)
+        where T: IntoIterator<Item = ty::Predicate<'tcx>>,
+              T::IntoIter: Clone,
+    {
+        let predicates = predicates.into_iter();
 
         debug!(
             "prove_predicates(predicates={:?}, location={:?})",
-            predicates_iter.by_ref().collect::<Vec<_>>(),
-            location
+            predicates.clone().collect::<Vec<_>>(),
+            location,
         );
         self.fully_perform_op(location.at_self(), |this| {
             let cause = this.misc(this.last_span);
-            let obligations = predicates_iter
+            let obligations = predicates
+                .into_iter()
                 .map(|p| traits::Obligation::new(cause.clone(), this.param_env, p))
                 .collect();
             Ok(InferOk {

