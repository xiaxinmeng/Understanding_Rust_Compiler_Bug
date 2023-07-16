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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 2010:
             let node = if let Some(node) = tcx.hir().find(hir_id) {
             } else {
             } else {
-                return Err("couldn't find id in the HIR map")
+                return Err("couldn't find id in the HIR map");
             match node {
             match node {
                 // Unique types created for closures participate in type privacy checking.
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 2030:
                 }) => ty::Visibility::from_hir(vis, hir_id, tcx),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_privacy/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                 // Visibilities of trait impl items are inherited from their traits
                 // and are not filled in resolve.
-                Node::ImplItem(_) => {
-                    match tcx.hir().get(tcx.hir().get_parent_item(hir_id)) {
-                        Node::Item(hir::Item {
-                            kind: hir::ItemKind::Impl { of_trait: Some(tr), .. },
-                            ..
-                        }) => tr.path.res.opt_def_id().map_or_else(
-                            || {
-                                tcx.sess.delay_span_bug(tr.path.span, "trait without a def-id");
-                                ty::Visibility::Public
-                            },
-                            |def_id| tcx.visibility(def_id),
-                        ),
-                        _ => return Err("the parent is not a trait impl"),
-                }
-                _ => return Err(
-                _ => return Err(
-                    "visibility table unexpectedly missing the def-id",
-                ),
+                Node::ImplItem(_) => match tcx.hir().get(tcx.hir().get_parent_item(hir_id)) {
+                    Node::Item(hir::Item {
+                        kind: hir::ItemKind::Impl { of_trait: Some(tr), .. },
+                        ..
+                    }) => tr.path.res.opt_def_id().map_or_else(
+                        || {
+                            tcx.sess.delay_span_bug(tr.path.span, "trait without a def-id");
+                            ty::Visibility::Public
+                        },
+                        |def_id| tcx.visibility(def_id),
+                    ),
+                    _ => return Err("the parent is not a trait impl"),
+                },
+                _ => return Err("visibility table unexpectedly missing the def-id"),
         }
     };
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:20
