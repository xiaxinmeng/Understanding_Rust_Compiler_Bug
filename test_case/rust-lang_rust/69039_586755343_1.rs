patch
--- a/src/librustc_mir/dataflow/impls/storage_liveness.rs
+++ b/src/librustc_mir/dataflow/impls/storage_liveness.rs
@@ -146,25 +146,58 @@ impl<'mir, 'tcx> BitDenotation<'tcx> for RequiresStorage<'mir, 'tcx> {
     fn before_terminator_effect(&self, sets: &mut GenKillSet<Local>, loc: Location) {
         self.check_for_borrow(sets, loc);
 
-        if let TerminatorKind::Call { destination: Some((Place { local, .. }, _)), .. } =
-            self.body[loc.block].terminator().kind
-        {
-            sets.gen(local);
+        match &self.body[loc.block].terminator().kind {
+            TerminatorKind::Call { destination: Some((Place { local, .. }, _)), .. }
+            | TerminatorKind::Yield { resume_arg: Place { local, .. }, .. } => {
+                sets.gen(*local);
+            }
+
+            // Nothing to do for these. Match exhaustively so this fails to compile when new
+            // variants are added.
+            TerminatorKind::Call { destination: None, .. }
+            | TerminatorKind::Abort
+            | TerminatorKind::Assert { .. }
+            | TerminatorKind::Drop { .. }
+            | TerminatorKind::DropAndReplace { .. }
+            | TerminatorKind::FalseEdges { .. }
+            | TerminatorKind::FalseUnwind { .. }
+            | TerminatorKind::GeneratorDrop
+            | TerminatorKind::Goto { .. }
+            | TerminatorKind::Resume
+            | TerminatorKind::Return
+            | TerminatorKind::SwitchInt { .. }
+            | TerminatorKind::Unreachable => {}
         }
     }
 
     fn terminator_effect(&self, sets: &mut GenKillSet<Local>, loc: Location) {
-        // For call terminators the destination requires storage for the call
-        // and after the call returns successfully, but not after a panic.
-        // Since `propagate_call_unwind` doesn't exist, we have to kill the
-        // destination here, and then gen it again in `propagate_call_return`.
-        if let TerminatorKind::Call { destination: Some((ref place, _)), .. } =
-            self.body[loc.block].terminator().kind
-        {
-            if let Some(local) = place.as_local() {
-                sets.kill(local);
+        match &self.body[loc.block].terminator().kind {
+            // For call terminators the destination requires storage for the call
+            // and after the call returns successfully, but not after a panic.
+            // Since `propagate_call_unwind` doesn't exist, we have to kill the
+            // destination here, and then gen it again in `propagate_call_return`.
+            TerminatorKind::Call { destination: Some((Place { local, .. }, _)), .. } => {
+                sets.kill(*local);
             }
+
+            // Nothing to do for these. Match exhaustively so this fails to compile when new
+            // variants are added.
+            TerminatorKind::Call { destination: None, .. }
+            | TerminatorKind::Yield { .. }
+            | TerminatorKind::Abort
+            | TerminatorKind::Assert { .. }
+            | TerminatorKind::Drop { .. }
+            | TerminatorKind::DropAndReplace { .. }
+            | TerminatorKind::FalseEdges { .. }
+            | TerminatorKind::FalseUnwind { .. }
+            | TerminatorKind::GeneratorDrop
+            | TerminatorKind::Goto { .. }
+            | TerminatorKind::Resume
+            | TerminatorKind::Return
+            | TerminatorKind::SwitchInt { .. }
+            | TerminatorKind::Unreachable => {}
         }
+
         self.check_for_move(sets, loc);
     }
 
-- 
2.25.0
