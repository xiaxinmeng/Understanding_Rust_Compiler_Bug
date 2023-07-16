diff
@@ -1706,8 +1706,7 @@ fn suggest_semicolon_removal(
 
     fn return_type_span(&self, obligation: &PredicateObligation<'tcx>) -> Option<Span> {
         let hir = self.tcx.hir();
-        let parent_node = hir.parent_id_by_def_id(obligation.cause.body_id);
-        let Some(hir::Node::Item(hir::Item { kind: hir::ItemKind::Fn(sig, ..), .. })) = hir.find(parent_node) else {
+        let Some(hir::Node::Item(hir::Item { kind: hir::ItemKind::Fn(sig, ..), .. })) = hir.find_by_def_id(obligation.cause.body_id) else {
             return None;
         };

