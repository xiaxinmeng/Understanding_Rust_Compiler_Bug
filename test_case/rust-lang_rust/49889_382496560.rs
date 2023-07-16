diff
diff --git a/src/librustc/ty/sty.rs b/src/librustc/ty/sty.rs
index 310fcbcfcb37..c34514178d65 100644
--- a/src/librustc/ty/sty.rs
+++ b/src/librustc/ty/sty.rs
@@ -885,8 +885,7 @@ impl<'a, 'gcx, 'tcx> ParamTy {
     }

     pub fn is_self(&self) -> bool {
-        if self.name == keywords::SelfType.name().as_str() {
-            assert_eq!(self.idx, 0);
+        if self.name == keywords::SelfType.name().as_str() && self.idx == 0 {
             true
         } else {
             false
