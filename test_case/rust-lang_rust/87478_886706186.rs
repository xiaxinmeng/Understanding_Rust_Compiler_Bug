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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2254:
                         Node::GenericParam(param) => {
                             for h in self.tcx.hir().parent_iter(param.hir_id) {
                                 break 'origin match h.1 {
-                                    Node::ImplItem(hir::ImplItem { kind: hir::ImplItemKind::TyAlias(..), generics, .. }) => SubOrigin::GAT(generics),
-                                    Node::ImplItem(hir::ImplItem { kind: hir::ImplItemKind::Fn(..), generics, .. }) => SubOrigin::Fn(generics),
-                                    Node::TraitItem(hir::TraitItem { kind: hir::TraitItemKind::Type(..), generics, ..}) => SubOrigin::GAT(generics),
-                                    Node::TraitItem(hir::TraitItem { kind: hir::TraitItemKind::Fn(..), generics, ..}) => SubOrigin::Fn(generics),
-                                    Node::Item(hir::Item { kind: hir::ItemKind::Trait(_, _, generics, _, _), .. }) => SubOrigin::Trait(generics),
-                                    Node::Item(hir::Item { kind: hir::ItemKind::Impl(hir::Impl { generics, .. }), .. }) => SubOrigin::Impl(generics),
-                                    Node::Item(hir::Item { kind: hir::ItemKind::Fn(_, generics, _), .. }) => SubOrigin::Fn(generics),
+                                    Node::ImplItem(hir::ImplItem {
+                                        kind: hir::ImplItemKind::TyAlias(..),
+                                        generics,
+                                        ..
+                                    }) => SubOrigin::GAT(generics),
+                                    Node::ImplItem(hir::ImplItem {
+                                        kind: hir::ImplItemKind::Fn(..),
+                                        generics,
+                                        ..
+                                    }) => SubOrigin::Fn(generics),
+                                    Node::TraitItem(hir::TraitItem {
+                                        kind: hir::TraitItemKind::Type(..),
+                                        generics,
+                                        ..
+                                    }) => SubOrigin::GAT(generics),
+                                    Node::TraitItem(hir::TraitItem {
+                                        kind: hir::TraitItemKind::Fn(..),
+                                        generics,
+                                        ..
+                                    }) => SubOrigin::Fn(generics),
+                                    Node::Item(hir::Item {
+                                        kind: hir::ItemKind::Trait(_, _, generics, _, _),
+                                        ..
+                                    }) => SubOrigin::Trait(generics),
+                                    Node::Item(hir::Item {
+                                        kind: hir::ItemKind::Impl(hir::Impl { generics, .. }),
+                                        ..
+                                    }) => SubOrigin::Impl(generics),
+                                    Node::Item(hir::Item {
+                                        kind: hir::ItemKind::Fn(_, generics, _),
+                                        ..
+                                    }) => SubOrigin::Fn(generics),
                                     _ => continue,
                             }
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2267:
                         }
-                        _ => {},
---
+                _ => {}
             }
             SubOrigin::Unknown
         };
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2297:
                     generics.where_clause.tail_span_for_suggestion(),
                     "consider adding a where clause".into(),
                     suggestion,
+                    Applicability::MaybeIncorrect,
                 );
                 err
             }
             }
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2304:
-            (ty::ReEarlyBound(ty::EarlyBoundRegion { name, .. })
-            | ty::ReFree(ty::FreeRegion { bound_region: ty::BrNamed(_, name), .. }), _) => {
+            (
+                ty::ReEarlyBound(ty::EarlyBoundRegion { name, .. })
+                | ty::ReFree(ty::FreeRegion { bound_region: ty::BrNamed(_, name), .. }),
+                _,
+            ) => {
                 // Does the required lifetime have a nice name we can print?
                 let mut err = struct_span_err!(
                     self.tcx.sess,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/canonical/canonicalizer.rs" "/checkout/compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs" "/checkout/compiler/rustc_infer/src/infer/fudge.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/util.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/mod.rs" "/checkout/compiler/rustc_infer/src/infer/canonical/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
