diff
diff --git a/src/librustdoc/html/layout.rs b/src/librustdoc/html/layout.rs
index 8c14d1bbe8..51e88c5bb0 100644
--- a/src/librustdoc/html/layout.rs
+++ b/src/librustdoc/html/layout.rs
@@ -72,7 +72,7 @@ r##"<!DOCTYPE html>
     <nav class="sub">
         <form class="search-form js-only">
             <div class="search-container">
-                <input class="search-input" name="search"
+                <input class="search-input" name="search" aria-controls="search"
                        autocomplete="off"
                        placeholder="Click or press ‘S’ to search, ‘?’ for more options…"
                        type="search">
@@ -81,7 +81,7 @@ r##"<!DOCTYPE html>
     </nav>
 
     <section id='main' class="content">{content}</section>
-    <section id='search' class="content hidden"></section>
+    <section id='search' class="content hidden" role="alert" aria-live="polite"></section>
 
     <section class="footer"></section>
