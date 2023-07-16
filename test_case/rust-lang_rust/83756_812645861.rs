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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/core.rs at line 464:
     if let Some(sized_trait_did) = ctxt.tcx.lang_items().sized_trait() {
         let mut sized_trait = build_external_trait(&mut ctxt, sized_trait_did);
         sized_trait.is_auto = true;
-        ctxt.external_traits.borrow_mut().insert(
-            sized_trait_did,
-            TraitWithExtraInfo { trait_: sized_trait, is_notable: false },
-        );
+        ctxt.external_traits
+            .borrow_mut()
+            .insert(sized_trait_did, TraitWithExtraInfo { trait_: sized_trait, is_notable: false });
 
 
     debug!("crate: {:?}", tcx.hir().krate());
Diff in /checkout/src/librustdoc/html/render/print_item.rs at line 10:
 use rustc_span::symbol::{kw, sym, Symbol};
 use super::{
 use super::{
-    collect_paths_for_type, document, ensure_trailing_slash, item_ty_to_strs, render_assoc_item,
-    render_assoc_items, render_attributes, render_impl, render_stability_since_raw, notable_traits_decl,
-    write_srclink, AssocItemLink, Context,
+    collect_paths_for_type, document, ensure_trailing_slash, item_ty_to_strs, notable_traits_decl,
+    render_assoc_item, render_assoc_items, render_attributes, render_impl,
+    render_stability_since_raw, write_srclink, AssocItemLink, Context,
 };
 use crate::clean::{self, GetDefId};
 use crate::formats::cache::Cache;
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1349:
         if let Some(impls) = cache.impls.get(&did) {
             for i in impls {
                 let impl_ = i.inner_impl();
-                if impl_.trait_.def_id().map_or(false, |d| {
-                    cache.traits.get(&d).map(|t| t.is_notable).unwrap_or(false)
-                }) {
+                if impl_
+                    .trait_
+                    .def_id()
+                    .map_or(false, |d| cache.traits.get(&d).map(|t| t.is_notable).unwrap_or(false))
+                {
                     if out.is_empty() {
                             &mut out,
                             &mut out,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/theme/tests.rs" "/checkout/src/librustdoc/lint.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/json/conversions.rs" "/checkout/src/librustdoc/theme.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
