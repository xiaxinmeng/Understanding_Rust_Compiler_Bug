diff
diff --git a/src/librustc/infer/error_reporting/need_type_info.rs b/src/librustc/infer/error_reporting/need_type_info.rs
index 7352c14..25bfb3e 100644
--- a/src/librustc/infer/error_reporting/need_type_info.rs
+++ b/src/librustc/infer/error_reporting/need_type_info.rs
@@ -131,7 +131,10 @@ impl<'a, 'gcx, 'tcx> InferCtxt<'a, 'gcx, 'tcx> {
             labels.clear();
             labels.push((pattern.span, format!("consider giving this closure parameter a type")));
         } else if let Some(pattern) = local_visitor.found_local_pattern {
-            if let Some(simple_name) = pattern.simple_name() {
+            // don't put internal desugared-loop identifier in user-facing
+            // message (Issue #51116)
+            let simple_name = pattern.simple_name().filter(|n| n.as_str() != "__next");
+            if let Some(simple_name) = simple_name {
                 labels.push((pattern.span, format!("consider giving `{}` a type", simple_name)));
             } else {
                 labels.push((pattern.span, format!("consider giving the pattern a type")));
