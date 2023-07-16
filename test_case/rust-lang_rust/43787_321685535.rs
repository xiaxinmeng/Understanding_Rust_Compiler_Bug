Rust
diff --git a/src/librustc/traits/select.rs b/src/librustc/traits/select.rs
index c2feb54c4d..16e075a1fc 100644
--- a/src/librustc/traits/select.rs
+++ b/src/librustc/traits/select.rs
@@ -893,7 +893,7 @@ impl<'cx, 'gcx, 'tcx> SelectionContext<'cx, 'gcx, 'tcx> {
         // being yet uninferred can create "spurious" EvaluatedToAmbig
         // and EvaluatedToOk.
         if result.is_stack_dependent() ||
-            ((result == EvaluatedToAmbig || result == EvaluatedToOk)
+            (result == EvaluatedToAmbig
              && trait_ref.has_closure_types())
         {
             return;
