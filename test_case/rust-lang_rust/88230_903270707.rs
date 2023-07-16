diff
$ gd f3e105f..bf88b11
diff --git a/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.fixed b/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.fixed
index 982407e2e25..c82bc369f43 100644
--- a/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.fixed
+++ b/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.fixed
@@ -3,7 +3,7 @@
 #![deny(rust_2021_incompatible_closure_captures)]
 //~^ NOTE: the lint level is defined here
 
-// Test cases for types that implement a insignificant drop (stlib defined)
+// Test cases for types that implement an insignificant drop (stlib defined)
 
 // `t` needs Drop because one of its elements needs drop,
 // therefore precise capture might affect drop ordering
 