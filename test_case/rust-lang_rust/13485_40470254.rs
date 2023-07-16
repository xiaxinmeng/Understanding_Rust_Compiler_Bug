 diff
diff --git a/src/librustdoc/html/render.rs b/src/librustdoc/html/render.rs
index ccee4f0..1874e4a 100644
--- a/src/librustdoc/html/render.rs
+++ b/src/librustdoc/html/render.rs
@@ -934,28 +934,8 @@ impl<'a> fmt::Show for Item<'a> {
             None => {}
         }

-        if self.cx.include_sources {
-            let mut path = Vec::new();
-            clean_srcpath(self.item.source.filename.as_bytes(), |component| {
-                path.push(component.to_owned());
-            });
-            let href = if self.item.source.loline == self.item.source.hiline {
-                format!("{}", self.item.source.loline)
-            } else {
-                format!("{}-{}", self.item.source.loline, self.item.source.hiline)
-            };
-            try!(write!(fmt.buf,
-                          "<a class='source'
-                              href='{root}src/{krate}/{path}.html\\#{href}'>\
-                              [src]</a>",
-                          root = self.cx.root_path,
-                          krate = self.cx.layout.krate,
-                          path = path.connect("/"),
-                          href = href));
-        }
-
         // Write the breadcrumb trail header for the top
-        try!(write!(fmt.buf, "<h1 class='fqn'>"));
+        try!(write!(fmt.buf, "\n<h1 class='fqn'>"));
         match self.item.inner {
             clean::ModuleItem(ref m) => if m.is_crate {
                     try!(write!(fmt.buf, "Crate "));
@@ -978,9 +958,30 @@ impl<'a> fmt::Show for Item<'a> {
             try!(write!(fmt.buf, "<a href='{}index.html'>{}</a>::",
                           trail, component.as_slice()));
         }
-        try!(write!(fmt.buf, "<a class='{}' href=''>{}</a></h1>",
+        try!(write!(fmt.buf, "<a class='{}' href=''>{}</a>",
                       shortty(self.item), self.item.name.get_ref().as_slice()));

+        if self.cx.include_sources {
+            let mut path = Vec::new();
+            clean_srcpath(self.item.source.filename.as_bytes(), |component| {
+                path.push(component.to_owned());
+            });
+            let href = if self.item.source.loline == self.item.source.hiline {
+                format!("{}", self.item.source.loline)
+            } else {
+                format!("{}-{}", self.item.source.loline, self.item.source.hiline)
+            };
+            try!(write!(fmt.buf,
+                          "<a class='source'\
+                              href='{root}src/{krate}/{path}.html\\#{href}'>\
+                              [src]</a>",
+                          root = self.cx.root_path,
+                          krate = self.cx.layout.krate,
+                          path = path.connect("/"),
+                          href = href));
+        }
+        try!(write!(fmt.buf, "</h1>\n"));
+
         match self.item.inner {
             clean::ModuleItem(ref m) => {
                 item_module(fmt.buf, self.cx, self.item, m.items.as_slice())
diff --git a/src/librustdoc/html/static/main.css b/src/librustdoc/html/static/main.css
index 41a3091..0db346b 100644
--- a/src/librustdoc/html/static/main.css
+++ b/src/librustdoc/html/static/main.css
@@ -12,8 +12,8 @@
 @font-face {
     font-family: 'Fira Sans';
     font-style: normal;
-    font-weight: 300;
-    src: local('Fira Sans Light'), url("http://rust-lang.org/fonts/FiraSans-Light.woff") format('woff');
+    font-weight: 400;
+    src: local('Fira Sans'), url("http://rust-lang.org/fonts/FiraSans-Regular.woff") format('woff');
 }
 @font-face {
     font-family: 'Fira Sans';
@@ -52,13 +52,12 @@

 body {
     color: #333;
-    min-height: 100%;
     min-width: 500px;
-    height: 100%;
     font: 18px "Heuristica", "Helvetica Neue", Helvetica, Arial, sans-serif;
-    line-height: 150%;
+    line-height: 1.4;
+   margin: 0;
     position: relative;
-    height: auto;
+   padding: 10px 15px 20px 15px;
     padding-bottom: 20px;
 }

@@ -78,7 +77,7 @@ h2, h3, h4 {
 h2 code, h3 code, h4 code, .block a {
     font-size: 1.2em;
 }
-h1, h2, h3, h4, section.sidebar, a.source {
+h1, h2, h3, h4, section.sidebar, a.source, .content a, .search-input {
     font-family: "Fira Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
 }

@@ -86,8 +85,12 @@ ul {
     padding-left: 25px;
 }

+p {
+    margin: 0 0 1em 0;
+}
+
 code, pre {
-    font-family: Menlo, Monaco, Consolas, "Inconsolata", "DejaVu Sans Mono", monospace;
+    font-family: Menlo, Monaco, Consolas, Inconsolata, "DejaVu Sans Mono", monospace;
 }
 pre {
     font-size: 15px;
@@ -151,7 +154,7 @@ nav.sub {
 }

 .content {
-    padding: 20px 40px;
+    padding: 20px 0;
 }

 .content pre { padding: 20px; }
@@ -201,10 +204,9 @@ nav.sub {
 .docblock h2 { font-size: 1.15em; }
 .docblock h3, .docblock h4, .docblock h5 { font-size: 1em; }

-.content .source { 
+.content .source {
     float: right;
-    font-weight: 500;
-    padding: 9px 15px;
+    font-size: 23px;
 }

 .content table {
@@ -260,7 +262,7 @@ nav.sum { text-align: right; }
 nav.sub form { display: inline; }

 nav, .content {
-    margin-left: 220px;
+    margin-left: 230px;
 }

 a {
@@ -290,18 +292,18 @@ a {
     margin-top: 5px;
     padding: 10px 16px;
     font-size: 17px;
-    font-weight: 300;
     box-shadow: 0 0 0 1px #e0e0e0, 0 0 0 2px transparent;
-    transition: background-color 50ms linear;
-    transition: border 500ms ease-out;
-    transition: box-shadow 500ms ease-out;
+    transition: border-color 300ms ease;
+    transition: border-radius 300ms ease-in-out;
+    transition: box-shadow 300ms ease-in-out;
 }

 .search-input:focus {
     border-color: #66afe9;
+    border-radius: 2px;
     border: 0;
     outline: 0;
-    box-shadow: 0 0 0 1px #078dd8, 0 0 0 2px #078dd8;
+    box-shadow: 0 0 8px #078dd8;
 }

 .search-results .desc {
@@ -397,7 +399,6 @@ h6.section-link:hover a:after {
     }

     nav.sub {
-        width: 85.5%;
         margin: 0 auto;
     }
 }
\ No newline at end of file
