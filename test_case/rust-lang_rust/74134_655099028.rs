patch
--- a/src/librustdoc/passes/collect_intra_doc_links.rs
+++ b/src/librustdoc/passes/collect_intra_doc_links.rs
@@ -773,6 +773,7 @@ impl<'a, 'tcx> DocFolder for LinkCollector<'a, 'tcx> {
 
                     let hir_id = self.cx.tcx.hir().as_local_hir_id(local);
                     if !self.cx.tcx.privacy_access_levels(LOCAL_CRATE).is_exported(hir_id)
+                        && (item.visibility == Visibility::Public)
                         && !self.cx.render_options.document_private
                     {
                         let item_name = item.name.as_deref().unwrap_or("<unknown>");
