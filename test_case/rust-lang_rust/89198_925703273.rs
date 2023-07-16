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
Diff in /checkout/src/librustdoc/clean/inline.rs at line 403:
                         hir::ImplItemKind::Fn(..) => ty::AssocKind::Fn,
                         hir::ImplItemKind::TyAlias(..) => ty::AssocKind::Type,
-                    let trait_item =
-                    let trait_item =
-                        tcx.associated_items(associated_trait.def_id).find_by_name_and_kind(
-                            tcx,
-                            item.ident,
-                            assoc_kind,
-                            associated_trait.def_id,
+                    let trait_item = tcx
+                    let trait_item = tcx
+                        .associated_items(associated_trait.def_id)
+                        .find_by_name_and_kind(tcx, item.ident, assoc_kind, associated_trait.def_id)
+                        .unwrap();
                     trace!("trait_item={:?}", trait_item);
                     !tcx.get_attrs(trait_item.def_id).lists(sym::doc).has_word(sym::hidden)
                 })
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/librustdoc/passes/doc_test_lints.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/passes/html_tags.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
