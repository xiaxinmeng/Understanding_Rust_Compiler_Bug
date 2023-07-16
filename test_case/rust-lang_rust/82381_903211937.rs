diff
diff --git a/src/librustdoc/html/render/context.rs b/src/librustdoc/html/render/context.rs
index 84bc1454f73..0df61a8b256 100644
--- a/src/librustdoc/html/render/context.rs
+++ b/src/librustdoc/html/render/context.rs
@@ -1,4 +1,4 @@
-use std::cell::RefCell;
+use std::cell::{RefCell, RefMut};
 use std::collections::BTreeMap;
 use std::error::Error as StdError;
 use std::io;
@@ -55,7 +55,10 @@
     /// publicly reused items to redirect to the right location.
     pub(super) render_redirect_pages: bool,
     /// The map used to ensure all generated 'id=' attributes are unique.
-    pub(super) id_map: RefCell<IdMap>,
+    ///
+    /// INVARIANT: there is always at least one map in this list (because `mod_item_out` is never
+    /// called without first calling `mod_item_in`).
+    pub(super) id_maps: Vec<RefCell<IdMap>>,
     /// Shared mutable state.
     ///
     /// Issue for improving the situation: [#82381][]
@@ -70,7 +73,7 @@
 
 // `Context` is cloned a lot, so we don't want the size to grow unexpectedly.
 #[cfg(target_arch = "x86_64")]
-rustc_data_structures::static_assert_size!(Context<'_>, 104);
+rustc_data_structures::static_assert_size!(Context<'_>, 88);
 
 /// Shared mutable state used in [`Context`] and elsewhere.
 crate struct SharedContext<'tcx> {
@@ -161,9 +164,12 @@ pub(super) fn sess(&self) -> &'tcx Session {
         &self.shared.tcx.sess
     }
 
+    pub(super) fn id_map(&self) -> RefMut<'_, IdMap> {
+        self.id_maps.last().unwrap().borrow_mut()
+    }
+
     pub(super) fn derive_id(&self, id: String) -> String {
-        let mut map = self.id_map.borrow_mut();
-        map.derive(id)
+        self.id_map().derive(id)
     }
 
     /// String representation of how to get back to the root path of the 'doc/'
@@ -502,7 +508,7 @@ fn init(
             current: Vec::new(),
             dst,
             render_redirect_pages: false,
-            id_map: RefCell::new(id_map),
+            id_maps: vec![RefCell::new(id_map)],
             shared: Rc::new(scx),
             include_sources,
         };
@@ -608,6 +614,7 @@ fn after_krate(&mut self) -> Result<(), Error> {
     }
 
     fn mod_item_in(&mut self, item: &clean::Item) -> Result<(), Error> {
+        self.id_maps.push(RefCell::new(IdMap::new()));
         // Stripped modules survive the rustdoc passes (i.e., `strip-private`)
         // if they contain impls for public types. These modules can also
         // contain items such as publicly re-exported structures.
@@ -653,6 +660,7 @@ fn mod_item_out(&mut self) -> Result<(), Error> {
         // Go back to where we were at
         self.dst.pop();
         self.current.pop();
+        self.id_maps.pop();
         Ok(())
     }
 
diff --git a/src/librustdoc/html/render/mod.rs b/src/librustdoc/html/render/mod.rs
index 7704abc9a72..31319df8086 100644
--- a/src/librustdoc/html/render/mod.rs
+++ b/src/librustdoc/html/render/mod.rs
@@ -485,14 +485,13 @@ fn document(w: &mut Buffer, cx: &Context<'_>, item: &clean::Item, parent: Option
 
 /// Render md_text as markdown.
 fn render_markdown(w: &mut Buffer, cx: &Context<'_>, md_text: &str, links: Vec<RenderedLink>) {
-    let mut ids = cx.id_map.borrow_mut();
     write!(
         w,
         "<div class=\"docblock\">{}</div>",
         Markdown(
             md_text,
             &links,
-            &mut ids,
+            &mut cx.id_map(),
             cx.shared.codes,
             cx.shared.edition(),
             &cx.shared.playground
@@ -622,7 +621,7 @@ fn short_item_info(
 
         if let Some(note) = note {
             let note = note.as_str();
-            let mut ids = cx.id_map.borrow_mut();
+            let mut ids = cx.id_map();
             let html = MarkdownHtml(
                 &note,
                 &mut ids,
@@ -661,13 +660,12 @@ fn short_item_info(
         message.push_str(&format!(" ({})", feature));
 
         if let Some(unstable_reason) = reason {
-            let mut ids = cx.id_map.borrow_mut();
             message = format!(
                 "<details><summary>{}</summary>{}</details>",
                 message,
                 MarkdownHtml(
                     &unstable_reason.as_str(),
-                    &mut ids,
+                    &mut cx.id_map(),
                     error_codes,
                     cx.shared.edition(),
                     &cx.shared.playground,
@@ -1537,14 +1535,13 @@ fn render_default_items(
         }
 
         if let Some(ref dox) = cx.shared.maybe_collapsed_doc_value(&i.impl_item) {
-            let mut ids = cx.id_map.borrow_mut();
             write!(
                 w,
                 "<div class=\"docblock\">{}</div>",
                 Markdown(
                     &*dox,
                     &i.impl_item.links(cx),
-                    &mut ids,
+                    &mut cx.id_map(),
                     cx.shared.codes,
                     cx.shared.edition(),
                     &cx.shared.playground
