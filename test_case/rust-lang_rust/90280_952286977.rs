plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 490:
         // all the parents in the loop below are also guaranteed to be modules.
         let mut module_def_id = macro_module_def_id;
         loop {
-            let changed_reachability =
-                self.update_macro_reachable(hir::OwnerId { def_id: module_def_id}, macro_module_def_id);
+            let changed_reachability = self.update_macro_reachable(
+                hir::OwnerId { def_id: module_def_id },
+                macro_module_def_id,
+            );
             if changed_reachability || module_def_id == CRATE_DEF_ID {
             }
             }
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 853:
             // Visit everything except for private impl items.
             hir::ItemKind::Impl(ref impl_) => {
                 if item_level.is_some() {
-                    self.reach(item.def_id.def_id, item_level).generics().predicates().ty().trait_ref();
+                    self.reach(item.def_id.def_id, item_level)
+                        .generics()
+                        .predicates()
+                        .ty()
+                        .trait_ref();
 
                     for impl_item_ref in impl_.items {
                         let impl_item_level = self.get(impl_item_ref.id.def_id.def_id);
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 1638:
                                 let impl_item = self.tcx.hir().impl_item(impl_item_ref.id);
                                 match impl_item.kind {
                                     hir::ImplItemKind::Const(..) | hir::ImplItemKind::Fn(..)
-                                        if self
-                                            .item_is_public(impl_item.def_id.def_id, &impl_item.vis) =>
+                                        if self.item_is_public(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/assert.rs" "/checkout/compiler/rustc_builtin_macros/src/env.rs" "/checkout/compiler/rustc_builtin_macros/src/util.rs" "/checkout/compiler/rustc_builtin_macros/src/lib.rs" "/checkout/compiler/rustc_builtin_macros/src/concat.rs" "/checkout/compiler/rustc_builtin_macros/src/derive.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_builtin_macros/src/llvm_asm.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                                            impl_item.def_id.def_id,
+                                            &impl_item.vis,
+                                        ) =>
                                         intravisit::walk_impl_item(self, impl_item)
                                     }
                                     }
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 2069:
             // A trait impl is public when both its type and its trait are public
             // Subitems of trait impls have inherited publicity.
             hir::ItemKind::Impl(ref impl_) => {
-                let impl_vis = ty::Visibility::of_impl(item.def_id.def_id, tcx, &Default::default());
+                let impl_vis =
+                    ty::Visibility::of_impl(item.def_id.def_id, tcx, &Default::default());
                 self.check(item.def_id.def_id, impl_vis).generics().predicates();
                 for impl_item_ref in impl_.items {
                     let impl_item_vis = if impl_.of_trait.is_none() {
