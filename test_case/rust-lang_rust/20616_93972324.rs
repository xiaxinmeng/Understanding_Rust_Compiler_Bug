 diff
diff --git a/src/librustc_typeck/check/method/suggest.rs b/src/librustc_typeck/check/method/suggest.rs
index c5ff8a1..ccf34bb 100644
--- a/src/librustc_typeck/check/method/suggest.rs
+++ b/src/librustc_typeck/check/method/suggest.rs
@@ -364,7 +364,7 @@ pub fn all_traits<'a>(ccx: &'a CrateCtxt) -> AllTraits<'a> {
 }

 pub struct AllTraits<'a> {
-    borrow: cell::Ref<'a Option<AllTraitsVec>>,
+    borrow: cell::Ref<'a, Option<AllTraitsVec>>,
     idx: usize
 }
