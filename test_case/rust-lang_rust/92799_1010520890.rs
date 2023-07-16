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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 398:
                 }
                 match tcx.type_of(did).kind() {
                     ty::Adt(def, _) if def.is_enum() => {
-                        if let Some(field) =
-                            def.all_fields().find(|f| f.name == variant_field_name)
+                        if let Some(field) = def.all_fields().find(|f| f.name == variant_field_name)
                         {
                             Ok((ty_res, Some(ItemFragment(FragmentKind::VariantField, field.did))))
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 770:
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 770:
                     ty::Adt(def, _) if !def.is_enum() => def,
                 };
-                let field = def
-                    .non_enum_variant()
-                    .fields
-                    .fields
-                    .iter()
-                    .find(|item| item.name == item_name)?;
+                let field =
+                    def.non_enum_variant().fields.iter().find(|item| item.name == item_name)?;
                 Some((root_res, ItemFragment(FragmentKind::StructField, field.did)))
             }
             Res::Def(DefKind::Trait, did) => tcx
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/fold.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/passes/unindent_comments/tests.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/bootstrap/job.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
