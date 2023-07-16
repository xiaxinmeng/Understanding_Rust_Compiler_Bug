
 Diff in /checkout/src/librustdoc/html/render/mod.rs at line 87:
 
 crate fn ensure_trailing_slash(v: &str) -> impl fmt::Display + '_ {
     crate::html::format::display_fn(move |f| {
-        if !v.ends_with('/') && !v.is_empty() {
-            write!(f, "{}/", v)
-        } else {
-            write!(f, "{}", v)
-        }
+        if !v.ends_with('/') && !v.is_empty() { write!(f, "{}/", v) } else { write!(f, "{}", v) }
     })
 }
 
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1870:
 }
 
 fn document_non_exhaustive_header(item: &clean::Item) -> &str {
-    if item.is_non_exhaustive() {
-        " (Non-exhaustive)"
-    } else {
-        ""
-    }
+    if item.is_non_exhaustive() { " (Non-exhaustive)" } else { "" }
 }
 
 fn document_non_exhaustive(w: &mut Buffer, item: &clean::Item) {
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 4144:
                                 if is_negative_impl { "!" } else { "" },
                                 out
                             );
-                            if links.insert(generated.clone()) {
-                                Some(generated)
-                            } else {
-                                None
-                            }
+                            if links.insert(generated.clone()) { Some(generated) } else { None }
                         } else {
                             None
                         }
