patch
fn drain_fulfillment_cx_or_panic<T>(
    infcx: &InferCtxt<'_, 'tcx>,
    fulfill_cx: &mut FulfillmentContext<'tcx>,
    result: &T,
) -> T
where
    T: TypeFoldable<'tcx>,
{
    debug!("drain_fulfillment_cx_or_panic()");

    // In principle, we only need to do this so long as `result`
    // contains unbound type parameters. It could be a slight
    // optimization to stop iterating early.
    if let Err(errors) = fulfill_cx.select_all_or_error(infcx) {
+        // `select_all_or_error` can return an empty `Vec<FulfillmentError>` when you have a bad
+        // interation of `impl Trait` in return position and a recursive ADT without indirection
+        // (#72554).
+        if !errors.is_empty() {
+            infcx.tcx.sess.delay_span_bug(
+                rustc_span::DUMMY_SP,
+                "`select_all_or_error` failed but no errors where present",
+            );
+        } else {
            bug!("encountered errors `{:?}` resolving bounds after type-checking", errors);
+        }
    }

    let result = infcx.resolve_vars_if_possible(result);
    infcx.tcx.erase_regions(&result)
}

