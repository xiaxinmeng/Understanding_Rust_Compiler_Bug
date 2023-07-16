diff
modified   src/librustc/traits/query/type_op/custom.rs
@@ -75,6 +75,14 @@ fn scrape_region_constraints<'gcx, 'tcx, R>(
 ) -> Fallible<(R, Option<Rc<Vec<QueryRegionConstraint<'tcx>>>>)> {
     let mut fulfill_cx = TraitEngine::new(infcx.tcx);
     let dummy_body_id = ObligationCause::dummy().body_id;
+
+    let pre_obligations = infcx.take_registered_region_obligations();
+    assert!(
+        pre_obligations.is_empty(),
+        "scrape_region_constraints: incoming region obligations = {:#?}",
+        pre_obligations,
+    );
+
     let InferOk { value, obligations } = infcx.commit_if_ok(|_| op())?;
     debug_assert!(obligations.iter().all(|o| o.cause.body_id == dummy_body_id));
     fulfill_cx.register_predicate_obligations(infcx, obligations);
