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
Diff in /checkout/compiler/rustc_typeck/src/check/check.rs at line 929:
     for impl_item in impl_items {
         let ty_impl_item = tcx.associated_item(tcx.hir().local_def_id(impl_item.hir_id));
 
-        let mut items = associated_items.filter_by_name(tcx, ty_impl_item.ident, impl_trait_ref.def_id);
+        let mut items =
+            associated_items.filter_by_name(tcx, ty_impl_item.ident, impl_trait_ref.def_id);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/check.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
         let (compatible_kind, ty_trait_item) = if let Some(ty_trait_item) = items.next() {
-
-            let is_compatible = |ty: &&ty::AssocItem| {
-                match (ty.kind, &impl_item.kind) {
-                    (ty::AssocKind::Const, hir::ImplItemKind::Const(..)) => true,
-                    (ty::AssocKind::Fn, hir::ImplItemKind::Fn(..)) => true,
-                    (ty::AssocKind::Type, hir::ImplItemKind::TyAlias(..)) => true,
-                }
-                }
+            let is_compatible = |ty: &&ty::AssocItem| match (ty.kind, &impl_item.kind) {
+                (ty::AssocKind::Const, hir::ImplItemKind::Const(..)) => true,
+                (ty::AssocKind::Fn, hir::ImplItemKind::Fn(..)) => true,
+                (ty::AssocKind::Type, hir::ImplItemKind::TyAlias(..)) => true,
             };
 
 
             // If we don't have a compatible item, we'll use the first one whose name matches
Diff in /checkout/compiler/rustc_typeck/src/check/check.rs at line 947:
             let mut compatible_kind = is_compatible(&ty_trait_item);
             let mut trait_item = ty_trait_item;
 
-            if  !compatible_kind {
+            if !compatible_kind {
                 if let Some(ty_trait_item) = items.find(is_compatible) {
                     compatible_kind = true;
                     trait_item = ty_trait_item;
Diff in /checkout/compiler/rustc_typeck/src/check/check.rs at line 1003:
             );
         } else {
         } else {
-            report_mismatch_error(tcx, ty_trait_item.def_id, impl_trait_ref, impl_item, &ty_impl_item);
+            report_mismatch_error(
+                tcx,
+                ty_trait_item.def_id,
+                impl_trait_ref,
+                impl_item,
+                &ty_impl_item,
+            );
     }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:15
