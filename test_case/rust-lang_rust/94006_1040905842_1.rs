diff
diff --git a/compiler/rustc_borrowck/src/diagnostics/region_errors.rs b/compiler/rustc_borrowck/s
index 87a27d2052b..a03d42f97c8 100644
--- a/compiler/rustc_borrowck/src/diagnostics/region_errors.rs
+++ b/compiler/rustc_borrowck/src/diagnostics/region_errors.rs
@@ -440,6 +440,9 @@ fn report_fnmut_error(

             let upvar_def_span = self.infcx.tcx.hir().span(defined_hir);
             let upvars_map = self.infcx.tcx.upvars_mentioned(def_id).unwrap();
+            println!("captured place: {captured_place:#?}");
+            println!("upvar definition span: {upvar_def_span:#?}");
+            println!("upvars_mentioned map: {upvars_map:#?}");
             let upvar_span = upvars_map.get(&captured_hir).unwrap().span;
             diag.span_label(upvar_def_span, "variable defined here");
             diag.span_label(upvar_span, "variable captured here");
