diff
$ git diff
diff --git a/src/librustdoc/html/static/css/rustdoc.css b/src/librustdoc/html/static/css/rustdoc.css
index 9e3ad667531..8cddd85854a 100644
--- a/src/librustdoc/html/static/css/rustdoc.css
+++ b/src/librustdoc/html/static/css/rustdoc.css
@@ -1729,7 +1729,7 @@ details.undocumented[open] > summary::before {
                width: calc(100% + 30px);
        }
 
-       .show-it {
+       .show-it, .sidebar-elems:focus-within {
+               z-index: 2;
                left: 0;
        }
