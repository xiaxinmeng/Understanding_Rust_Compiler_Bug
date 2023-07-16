patch
--- a/src/bootstrap/doc.rs
+++ b/src/bootstrap/doc.rs
@@ -375,7 +375,7 @@ impl Step for Standalone {
                up_to_date(&footer, &html) &&
                up_to_date(&favicon, &html) &&
                up_to_date(&full_toc, &html) &&
-               up_to_date(&version_info, &html) &&
+               (builder.config.dry_run || up_to_date(&version_info, &html)) &&
                (builder.config.dry_run || up_to_date(&rustdoc, &html)) {
                 continue
             }
