plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 33:
 use std::mem;
 use std::ops::Range;
 
-use crate::{clean::{self, utils::find_nearest_parent_module, Crate, Item, ItemLink, PrimitiveType}, html::render::cache::ExternalLocation};
 use crate::core::DocContext;
 use crate::fold::DocFolder;
 use crate::html::markdown::{markdown_links, MarkdownLink};
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 40:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 use crate::passes::Pass;
+use crate::{
+    clean::{self, utils::find_nearest_parent_module, Crate, Item, ItemLink, PrimitiveType},
+    html::render::cache::ExternalLocation,
 
 use super::span_of_attrs;
 
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2052:
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2052:
 }
 
 /// Report a link to an item that will not have documentation generated.
-fn missing_docs_for_link(cx: &DocContext<'_>, item: &Item, destination_id: DefId, path_str: &str, dox: &str, link: &MarkdownLink) {
+fn missing_docs_for_link(
+    cx: &DocContext<'_>,
+    item: &Item,
+    destination_id: DefId,
+    path_str: &str,
+    dox: &str,
+    link: &MarkdownLink,
     let sym;
     let sym;
     let item_name = match item.name {
         Some(name) => {
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2061:
         }
         None => "<unknown>",
-    let msg =
-    let msg =
-        format!("documentation for `{}` links to item `{}` which will not have documentation generated", item_name, path_str);
+    let msg = format!(
+        "documentation for `{}` links to item `{}` which will not have documentation generated",
+        item_name, path_str
+    );
 
     report_diagnostic(cx, PRIVATE_INTRA_DOC_LINKS, &msg, item, dox, &link.range, |diag, sp| {
-        use rustc_middle::ty::DefIdTree;
         use crate::clean::types::{AttributesExt, NestedAttributesExt};
+        use rustc_middle::ty::DefIdTree;
 
         if let Some(sp) = sp {
             diag.span_label(sp, "this item is will not be documented");
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2073:
 
 
-        if matches!(cx.cache.extern_locations.get(&destination_id.krate), Some((.., ExternalLocation::Unknown))) {
-            diag.note(&format!("`{}` is in the crate `{}`, which will not be documented", path_str, cx.tcx.crate_name(destination_id.krate)));
+        if matches!(
+            cx.cache.extern_locations.get(&destination_id.krate),
+            Some((.., ExternalLocation::Unknown))
+        ) {
+            diag.note(&format!(
+                "`{}` is in the crate `{}`, which will not be documented",
+                path_str,
+                cx.tcx.crate_name(destination_id.krate)
             return;
         }
 
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2080:
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2080:
-        let hidden_attr = cx.tcx.get_attrs(destination_id).lists(sym::doc).get_word_attr(sym::hidden);
+        let hidden_attr =
+            cx.tcx.get_attrs(destination_id).lists(sym::doc).get_word_attr(sym::hidden);
         if let Some(attr) = hidden_attr.0 {
             diag.span_label(attr.span(), &format!("`{}` is marked as `#[doc(hidden)]`", path_str));
             return;
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2089:
                 let name = cx.tcx.item_name(parent);
                 let (_, description) = cx.tcx.article_and_description(parent);
                 let span = cx.tcx.def_span(parent);
-                diag.span_label(span, &format!("{} is in the {} `{}`, which is marked as `#[doc(hidden)]`", path_str, description, name));
+                diag.span_label(
+                    span,
+                    &format!(
+                        "{} is in the {} `{}`, which is marked as `#[doc(hidden)]`",
+                        path_str, description, name
+                );
                 return;
             }
             current = parent;
             current = parent;
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 2096:
         }
 
-        diag.note(&format!("`{}` may be in a private module with all re-exports marked as `#[doc(no_inline)]`", path_str));
+        diag.note(&format!(
+            "`{}` may be in a private module with all re-exports marked as `#[doc(no_inline)]`",
+        ));
     });
 }
 
