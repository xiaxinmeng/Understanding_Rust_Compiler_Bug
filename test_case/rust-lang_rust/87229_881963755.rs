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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 2052:
         }) => ty::Visibility::from_hir(vis, hir_id, tcx),
         // Visibilities of trait impl items are inherited from their traits
         // and are not filled in resolve.
-        Node::ImplItem(impl_item) => {
-            match tcx.hir().get(tcx.hir().get_parent_item(hir_id)) {
-                Node::Item(hir::Item {
-                    kind: hir::ItemKind::Impl(hir::Impl { of_trait: Some(tr), .. }),
-                    ..
-                }) => tr.path.res.opt_def_id().map_or_else(
-                    || {
-                        tcx.sess.delay_span_bug(tr.path.span, "trait without a def-id");
-                        ty::Visibility::Public
-                    },
-                    |def_id| tcx.visibility(def_id),
-                ),
-                _ => span_bug!(impl_item.span, "the parent is not a trait impl"),
-        }
-        }
+        Node::ImplItem(impl_item) => match tcx.hir().get(tcx.hir().get_parent_item(hir_id)) {
+            Node::Item(hir::Item {
+                kind: hir::ItemKind::Impl(hir::Impl { of_trait: Some(tr), .. }),
+                ..
+            }) => tr.path.res.opt_def_id().map_or_else(
+                || {
+                    tcx.sess.delay_span_bug(tr.path.span, "trait without a def-id");
+                    ty::Visibility::Public
+                },
+                |def_id| tcx.visibility(def_id),
+            ),
+            _ => span_bug!(impl_item.span, "the parent is not a trait impl"),
         _ => span_bug!(
         _ => span_bug!(
             tcx.def_span(def_id),
             "visibility table unexpectedly missing a def-id: {:?}",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_ast_passes/src/ast_validation.rs" "/checkout/compiler/rustc_ast_passes/src/node_count.rs" "/checkout/compiler/rustc_ast_passes/src/show_span.rs" "/checkout/compiler/rustc_ast_passes/src/lib.rs" "/checkout/compiler/rustc_ast_passes/src/feature_gate.rs" "/checkout/compiler/rustc_arena/src/lib.rs" "/checkout/compiler/rustc_arena/src/tests.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
