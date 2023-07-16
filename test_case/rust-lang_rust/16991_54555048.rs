 diff
diff --git a/src/librustdoc/html/layout.rs b/src/librustdoc/html/layout.rs
index 896d070..df84b9b 100644
--- a/src/librustdoc/html/layout.rs
+++ b/src/librustdoc/html/layout.rs
@@ -12,6 +12,7 @@ use std::fmt;
 use std::io;

 use externalfiles::ExternalHtml;
+use html::markdown;

 #[deriving(Clone)]
 pub struct Layout {
@@ -34,6 +35,8 @@ pub fn render<T: fmt::Show, S: fmt::Show>(
     dst: &mut io::Writer, layout: &Layout, page: &Page, sidebar: &S, t: &T)
     -> io::IoResult<()>
 {
+    markdown::math_seen.replace(Some(false));
+
     write!(dst,
 r##"<!DOCTYPE html>
 <html lang="en">
@@ -156,6 +159,12 @@ r##"<!DOCTYPE html>
     } else {
         format!(r#"<script src="{}playpen.js"></script>"#, page.root_path)
     },
+    // this must be last so that `math_seen` captures all possible $$'s on this page.
+    mathjax_js = if layout.use_mathjax && markdown::math_seen.get().map_or(false, |x| *x) {
+        "..."
+    } else {
+        ""
+    }
     )
 }

diff --git a/src/librustdoc/html/markdown.rs b/src/librustdoc/html/markdown.rs
index 305c184..f205078 100644
--- a/src/librustdoc/html/markdown.rs
+++ b/src/librustdoc/html/markdown.rs
@@ -101,6 +101,8 @@ struct MyOpaque {
     dfltblk: extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer,
                            *const hoedown_buffer, *mut libc::c_void),
     toc_builder: Option<TocBuilder>,
+    math_enabled: bool,
+    math_seen: bool,
 }

 #[repr(C)]
@@ -151,6 +153,7 @@ local_data_key!(used_header_map: RefCell<HashMap<String, uint>>)
 local_data_key!(test_idx: Cell<uint>)
 // None == render an example, but there's no crate name
 local_data_key!(pub playground_krate: Option<String>)
+local_data_key!(pub math_seen: bool)

 pub fn render(w: &mut fmt::Formatter, s: &str, print_toc: bool) -> fmt::Result {
     extern fn block(ob: *mut hoedown_buffer, text: *const hoedown_buffer,
@@ -274,16 +277,48 @@ pub fn render(w: &mut fmt::Formatter, s: &str, print_toc: bool) -> fmt::Result {
         text.with_c_str(|p| unsafe { hoedown_buffer_puts(ob, p) });
     }

+    extern fn math(ob: *mut hoedown_buffer, text: *const hoedown_buffer,
+                   display_mode: libc::c_int, opaque: *mut libc::c_void) -> libc::c_int {
+
+        let opaque = opaque as *mut hoedown_html_renderer_state;
+        let opaque = unsafe { &mut *((*opaque).opaque as *mut MyOpaque) };
+
+        opaque.math_seen = true;
+
+        let (open, close) = if !opaque.math_enabled {
+            ("$$", "$$")
+        } else if displaymode == 1 {
+            ("\\[", "\\]")
+        } else {
+            ("\\(", "\\)")
+        };
+
+        open.with_c_str(|open| {
+            close.with_c_str(|close| {
+                unsafe {
+                    hoedown_buffer_puts(ob, open, 2);
+                    escape_html(ob, (*text).data, (*text).size);
+                    hoedown_buffer_puts(ob, close, 2);
+                }
+            })
+        });
+
+        1
+    }
+
     unsafe {
         let ob = hoedown_buffer_new(DEF_OUNIT);
         let renderer = hoedown_html_renderer_new(0, 0);
         let mut opaque = MyOpaque {
             dfltblk: (*renderer).blockcode.unwrap(),
-            toc_builder: if print_toc {Some(TocBuilder::new())} else {None}
+            toc_builder: if print_toc {Some(TocBuilder::new())} else {None},
+            math_enabled: use_mathjax.get().map_or(false, |x| *x),
+            math_seen: false,
         };
         (*(*renderer).opaque).opaque = &mut opaque as *mut _ as *mut libc::c_void;
         (*renderer).blockcode = Some(block);
         (*renderer).header = Some(header);
+        (*renderer).math = Some(math);

         let document = hoedown_document_new(renderer, HOEDOWN_EXTENSIONS, 16);
         hoedown_document_render(document, ob, s.as_ptr(),
@@ -303,6 +338,10 @@ pub fn render(w: &mut fmt::Formatter, s: &str, print_toc: bool) -> fmt::Result {
             });
         }
         hoedown_buffer_free(ob);
+
+        let old = math_seen.get().map_or(false, |x| *x);
+        math_seen.replace(Some(old || opaque.math_seen));
+
         ret
     }
 }
