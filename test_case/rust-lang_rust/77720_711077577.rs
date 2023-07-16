patch
@@ -521,12 +521,7 @@ impl<'cx, 'tcx> SelectionContext<'cx, 'tcx> {
                             result
                         }
                         Ok(Ok(None)) => Ok(EvaluatedToAmbig),
-                        // EvaluatedToRecur might also be acceptable here, but use
-                        // Unknown for now because it means that we won't dismiss a
-                        // selection candidate solely because it has a projection
-                        // cycle. This is closest to the previous behavior of
-                        // immediately erroring.
-                        Ok(Err(project::InProgress)) => Ok(EvaluatedToUnknown),
+                        Ok(Err(project::InProgress)) => Ok(EvaluatedToRecur),
                         Err(_) => Ok(EvaluatedToErr),
                     }
                 }
@@ -1387,9 +1382,7 @@ impl<'cx, 'tcx> SelectionContext<'cx, 'tcx> {
             | (ObjectCandidate(i), ObjectCandidate(j)) => {
                 // Arbitrarily pick the lower numbered candidate for backwards
                 // compatibility reasons. Don't let this affect inference.
-                other.evaluation.must_apply_modulo_regions()
-                    && !needs_infer
-                    && (i < j || !victim.evaluation.must_apply_modulo_regions())
+                i < j && !needs_infer
             }
             (ObjectCandidate(_), ProjectionCandidate(_))
             | (ProjectionCandidate(_), ObjectCandidate(_)) => {
