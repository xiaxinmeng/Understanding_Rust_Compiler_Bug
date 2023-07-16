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
Diff in /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs at line 526:
         let hir_map = self.infcx.tcx.hir();
         let my_def = self.body.source.def_id();
         let my_hir = hir_map.local_def_id_to_hir_id(my_def.as_local().unwrap());
-        let td = if let Some(a) = self.infcx.tcx.impl_of_method(my_def).and_then(|x| {
-            self.infcx.tcx.trait_id_of_impl(x)
-        }) {
+        let td = if let Some(a) =
+            self.infcx.tcx.impl_of_method(my_def).and_then(|x| self.infcx.tcx.trait_id_of_impl(x))
             a
         } else {
             return (false, None);
             return (false, None);
Diff in /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs at line 535:
         };
-        (true, td.as_local().and_then(|tld| {
-            let h = hir_map.local_def_id_to_hir_id(tld);
-            match hir_map.find(h) {
-                Some(Node::Item(hir::Item {
-                    kind: hir::ItemKind::Trait(
-                        _, _, _, _,
-                    ),
-                    ..
-                })) => {
-                })) => {
-                    let mut f_in_trait_opt = None;
-                    for hir::TraitItemRef { id: fi, kind: k, .. } in *items {
-                        let hi = fi.hir_id();
-                        if !matches!(k, hir::AssocItemKind::Fn { .. }) {
+        (
+            true,
+            true,
+            td.as_local().and_then(|tld| {
+                let h = hir_map.local_def_id_to_hir_id(tld);
+                match hir_map.find(h) {
+                    Some(Node::Item(hir::Item {
+                        kind: hir::ItemKind::Trait(_, _, _, _, items),
+                    })) => {
+                    })) => {
+                        let mut f_in_trait_opt = None;
+                        for hir::TraitItemRef { id: fi, kind: k, .. } in *items {
+                            let hi = fi.hir_id();
+                            if !matches!(k, hir::AssocItemKind::Fn { .. }) {
+                            }
+                            }
+                            if hir_map.name(hi) != hir_map.name(my_hir) {
+                            }
+                            }
+                            f_in_trait_opt = Some(hi);
+                            break;
                         }
-                        if hir_map.name(hi) != hir_map.name(my_hir) {
-                        }
-                        }
-                        f_in_trait_opt = Some(hi);
-                        break;
-                    }
-                    f_in_trait_opt.and_then(|f_in_trait| {
-                        match hir_map.find(f_in_trait) {
+                        f_in_trait_opt.and_then(|f_in_trait| match hir_map.find(f_in_trait) {
                             Some(Node::TraitItem(hir::TraitItem {
-                                kind: hir::TraitItemKind::Fn(hir::FnSig {
-                                    decl: hir::FnDecl {
-                                        inputs,
-                                    },
-                                    ..
-                                }, _),
+                                kind:
+                                kind:
+                                    hir::TraitItemKind::Fn(
+                                        hir::FnSig { decl: hir::FnDecl { inputs, .. }, .. },
+                                        _,
                                 ..
                             })) => {
                             })) => {
                                 let hir::Ty { span, .. } = inputs[local.index() - 1];
Diff in /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs at line 571:
                                 Some(span)
+                            }
                             _ => None,
-                        }
-                    })
---
+            }),
+        )
     }
 
     // point to span of upvar making closure call require mutable borrow
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/region_errors.rs" "/checkout/compiler/rustc_mir/src/borrow_check/region_infer/values.rs" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_mir/src/borrow_check/region_infer/dump_mir.rs" "/checkout/compiler/rustc_mir/src/borrow_check/region_infer/graphviz.rs" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/var_name.rs" "/checkout/compiler/rustc_mir/src/util/generic_graphviz.rs" "/checkout/compiler/rustc_mir/src/borrow_check/region_infer/reverse_sccs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
