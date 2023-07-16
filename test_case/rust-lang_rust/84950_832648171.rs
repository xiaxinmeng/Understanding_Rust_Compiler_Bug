diff
diff --git a/src/librustdoc/passes/collect_intra_doc_links.rs b/src/librustdoc/passes/collect_intra_doc_links.rs
index c5fb54e52c7..c5f344630a6 100644
--- a/src/librustdoc/passes/collect_intra_doc_links.rs
+++ b/src/librustdoc/passes/collect_intra_doc_links.rs
@@ -2018,11 +2018,8 @@ fn disambiguator_error(
 ) {
     diag_info.link_range = disambiguator_range;
     report_diagnostic(cx.tcx, BROKEN_INTRA_DOC_LINKS, msg, &diag_info, |diag, _sp| {
-        let msg = format!(
-            "see https://doc.rust-lang.org/{}/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators \
-             for more info about disambiguators",
-            crate::doc_rust_lang_org_channel(),
-        );
+        let msg = "see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators \
+             for more info about disambiguators";
         diag.note(&msg);
     });
 }
