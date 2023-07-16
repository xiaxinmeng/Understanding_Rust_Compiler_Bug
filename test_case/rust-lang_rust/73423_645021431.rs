patch
diff --git a/src/librustdoc/clean/types.rs b/src/librustdoc/clean/types.rs
index 5c76c840b1d..1eb52c8f7bb 100644
--- a/src/librustdoc/clean/types.rs
+++ b/src/librustdoc/clean/types.rs
@@ -55,6 +55,7 @@ pub struct Crate {
     pub external_traits: Rc<RefCell<FxHashMap<DefId, Trait>>>,
     pub masked_crates: FxHashSet<CrateNum>,
     pub collapsed: bool,
+    pub is_no_std: bool,
 }
 
 #[derive(Clone, Debug)]
@@ -114,8 +115,8 @@ impl Item {
         self.attrs.collapsed_doc_value()
     }
 
-    pub fn links(&self) -> Vec<(String, String)> {
-        self.attrs.links(&self.def_id.krate)
+    pub fn links(&self, is_no_std: bool) -> Vec<(String, String)> {
+        self.attrs.links(&self.def_id.krate, is_no_std)
     }
 
     pub fn is_crate(&self) -> bool {
@@ -598,7 +599,7 @@ impl Attributes {
     /// Gets links as a vector
     ///
     /// Cache must be populated before call
-    pub fn links(&self, krate: &CrateNum) -> Vec<(String, String)> {
+    pub fn links(&self, krate: &CrateNum, is_no_std: bool) -> Vec<(String, String)> {
         use crate::html::format::href;
 
         self.links
@@ -630,12 +631,14 @@ impl Attributes {
                             };
                             // This is a primitive so the url is done "by hand".
                             let tail = fragment.find('#').unwrap_or_else(|| fragment.len());
+                            let std = if is_no_std {"core"} else {"std"};
                             Some((
                                 s.clone(),
                                 format!(
-                                    "{}{}std/primitive.{}.html{}",
+                                    "{}{}{}/primitive.{}.html{}",
                                     url,
                                     if !url.ends_with('/') { "/" } else { "" },
+                                    std,
                                     &fragment[..tail],
                                     &fragment[tail..]
                                 ),
diff --git a/src/librustdoc/clean/utils.rs b/src/librustdoc/clean/utils.rs
index c4e4802db6c..cb6b7d26f7d 100644
--- a/src/librustdoc/clean/utils.rs
+++ b/src/librustdoc/clean/utils.rs
@@ -10,6 +10,7 @@ use crate::core::DocContext;
 
 use itertools::Itertools;
 use rustc_data_structures::fx::FxHashSet;
+use rustc_ast::attr;
 use rustc_hir as hir;
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::{DefId, LOCAL_CRATE};
@@ -87,6 +88,7 @@ pub fn krate(mut cx: &mut DocContext<'_>) -> Crate {
         }));
     }
 
+    let is_no_std = attr::contains_name(&krate.item.attrs, sym::no_std);
     Crate {
         name,
         version: None,
@@ -97,6 +99,7 @@ pub fn krate(mut cx: &mut DocContext<'_>) -> Crate {
         external_traits: cx.external_traits.clone(),
         masked_crates,
         collapsed: false,
+        is_no_std,
     }
 }
 
diff --git a/src/librustdoc/html/render.rs b/src/librustdoc/html/render.rs
index 07631093edd..abc6ee5de17 100644
--- a/src/librustdoc/html/render.rs
+++ b/src/librustdoc/html/render.rs
@@ -161,6 +161,7 @@ struct Context {
     id_map: Rc<RefCell<IdMap>>,
     pub shared: Arc<SharedContext>,
     pub cache: Arc<Cache>,
+    pub is_no_std: bool,
 }
 
 crate struct SharedContext {
@@ -556,6 +557,7 @@ pub fn run(
         id_map: Rc::new(RefCell::new(id_map)),
         shared: Arc::new(scx),
         cache: cache.clone(),
+        is_no_std: krate.is_no_std,
     };
 
     // Freeze the cache now that the index has been built. Put an Arc into TLS
@@ -1890,7 +1892,7 @@ fn document_short(
         } else {
             plain_summary_line(Some(s))
         };
-        render_markdown(w, cx, &markdown, item.links(), prefix, is_hidden);
+        render_markdown(w, cx, &markdown, item.links(cx.is_no_std), prefix, is_hidden);
     } else if !prefix.is_empty() {
         write!(
             w,
@@ -1904,7 +1906,7 @@ fn document_short(
 fn document_full(w: &mut Buffer, item: &clean::Item, cx: &Context, prefix: &str, is_hidden: bool) {
     if let Some(s) = cx.shared.maybe_collapsed_doc_value(item) {
         debug!("Doc block: =====\n{}\n=====", s);
-        render_markdown(w, cx, &*s, item.links(), prefix, is_hidden);
+        render_markdown(w, cx, &*s, item.links(cx.is_no_std), prefix, is_hidden);
     } else if !prefix.is_empty() {
         write!(
             w,
@@ -2158,7 +2160,7 @@ fn item_module(w: &mut Buffer, cx: &Context, item: &clean::Item, items: &[clean:
                        </tr>",
                     name = *myitem.name.as_ref().unwrap(),
                     stab_tags = stability_tags(myitem),
-                    docs = MarkdownSummaryLine(doc_value, &myitem.links()).to_string(),
+                    docs = MarkdownSummaryLine(doc_value, &myitem.links(cx.is_no_std)).to_string(),
                     class = myitem.type_(),
                     add = add,
                     stab = stab.unwrap_or_else(String::new),
@@ -3616,7 +3618,7 @@ fn render_impl(
                 "<div class='docblock'>{}</div>",
                 Markdown(
                     &*dox,
-                    &i.impl_item.links(),
+                    &i.impl_item.links(cx.is_no_std),
                     &mut ids,
                     cx.shared.codes,
                     cx.shared.edition,
