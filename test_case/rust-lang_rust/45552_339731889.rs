diff
diff --git a/src/theme/index.hbs b/src/theme/index.hbs
index 9f735e6cd..3b4dbd12c 100644
--- a/src/theme/index.hbs
+++ b/src/theme/index.hbs
@@ -88,7 +88,9 @@
                     <h1 class="menu-title">{{ book_title }}</h1>
 
                     <div class="right-buttons">
-                        <i id="print-button" class="fa fa-print" title="Print this book"></i>
+                        <a href="print.html">
+                            <i id="print-button" class="fa fa-print" title="Print this book"></i>
+                        </a>
                     </div>
                 </div>
