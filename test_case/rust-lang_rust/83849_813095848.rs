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
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 376:
     ) -> Option<(Res, String)> {
         let tcx = self.cx.tcx;
-        prim_ty
-        prim_ty
-            .impls(tcx)
-            .into_iter()
-            .find_map(|&impl_| {
-                tcx.associated_items(impl_)
-                    .find_by_name_and_namespace(tcx, Ident::with_dummy_span(item_name), ns, impl_)
-                    .map(|item| {
-                        let kind = item.kind;
-                        self.kind_side_channel.set(Some((kind.as_def_kind(), item.def_id)));
-                        match kind {
-                            ty::AssocKind::Fn => "method",
-                            ty::AssocKind::Const => "associatedconstant",
-                            ty::AssocKind::Type => "associatedtype",
-                    })
-                    })
-                    .map(|out| {
-                        (
-                            Res::Primitive(prim_ty),
-                            format!("{}#{}.{}", prim_ty.as_str(), out, item_name),
-                    })
-            })
-            })
+        prim_ty.impls(tcx).into_iter().find_map(|&impl_| {
+            tcx.associated_items(impl_)
+                .find_by_name_and_namespace(tcx, Ident::with_dummy_span(item_name), ns, impl_)
+                .map(|item| {
+                    let kind = item.kind;
+                    self.kind_side_channel.set(Some((kind.as_def_kind(), item.def_id)));
+                    match kind {
+                        ty::AssocKind::Fn => "method",
+                        ty::AssocKind::Const => "associatedconstant",
+                        ty::AssocKind::Type => "associatedtype",
+                })
+                })
+                .map(|out| {
+                    (Res::Primitive(prim_ty), format!("{}#{}.{}", prim_ty.as_str(), out, item_name))
+        })
     }
 
     /// Resolves a string as a macro.
     /// Resolves a string as a macro.
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 515:
 
         self.resolve_path(&path_root, TypeNS, module_id)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/compiler/rustc_mir_build/src/thir/arena.rs" "/checkout/src/librustdoc/lint.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/usefulness.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
             .and_then(|ty_res| {
-                let (res, fragment, side_channel) = self.resolve_associated_item(ty_res, item_name, ns, module_id)?;
+                let (res, fragment, side_channel) =
+                    self.resolve_associated_item(ty_res, item_name, ns, module_id)?;
                 let result = if extra_fragment.is_some() {
-                    Err(ErrorKind::AnchorFailure(AnchorFailure::RustdocAnchorConflict(
-                    )))
-                    )))
+                    Err(ErrorKind::AnchorFailure(AnchorFailure::RustdocAnchorConflict(res)))
                 } else {
                     // HACK(jynelson): `clean` expects the type, not the associated item
                     // but the disambiguator logic expects the associated item.
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 559:
         let tcx = self.cx.tcx;
         match root_res {
         match root_res {
-            Res::Primitive(prim) => {
-                self.resolve_primitive_associated_item(prim, ns, item_name)
-                    .map(|(x, y)| (x, y, None))
-            }
+            Res::Primitive(prim) => self
+                .resolve_primitive_associated_item(prim, ns, item_name)
+                .map(|(x, y)| (x, y, None)),
             Res::Def(
                 DefKind::Struct
                 | DefKind::Union
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 608:
                     // HACK(jynelson): `clean` expects the type, not the associated item
                     // but the disambiguator logic expects the associated item.
                     // Store the kind in a side channel so that only the disambiguator logic looks at it.
-                    return Some((root_res, format!("{}.{}", out, item_name), Some((kind.as_def_kind(), id))));
+                    return Some((
+                        root_res,
+                        format!("{}.{}", out, item_name),
+                        Some((kind.as_def_kind(), id)),
                 }
 
 
                 if ns != Namespace::ValueNS {
Build completed unsuccessfully in 0:00:13
