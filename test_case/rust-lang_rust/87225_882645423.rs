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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 2170:
                 self.tcx.for_each_relevant_impl(
                     parent_def_id,
                     parent_trait_ref.self_ty().skip_binder(),
-                    |impl_def_id| {
-                        match self.tcx.hir().get_if_local(impl_def_id) {
-                            Some(Node::Item(hir::Item {
-                                kind: hir::ItemKind::Impl(hir::Impl { .. }),
-                            })) => {
-                            })) => {
-                                candidates.push(impl_def_id);
-                            _ => {}
-                            _ => {}
+                    |impl_def_id| match self.tcx.hir().get_if_local(impl_def_id) {
+                        Some(Node::Item(hir::Item {
+                            kind: hir::ItemKind::Impl(hir::Impl { .. }),
+                        })) => {
+                        })) => {
+                            candidates.push(impl_def_id);
+                        _ => {}
                     },
                 );
                 );
                 match &candidates[..] {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_ast/src/entry.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/subtype.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/prove_predicate.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
