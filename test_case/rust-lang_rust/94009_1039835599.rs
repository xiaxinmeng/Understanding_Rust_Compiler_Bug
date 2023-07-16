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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1053:
                     let type_ = hir_ty.clean(cx);
                     let generics = self.generics.clean(cx);
                     let item_type = hir_ty_to_ty(cx.tcx, hir_ty).clean(cx);
-                    TypedefItem(
-                        Typedef {
-                            type_,
-                            generics,
-                            item_type: Some(item_type),
-                        true,
-                    )
-                    )
+                    TypedefItem(Typedef { type_, generics, item_type: Some(item_type) }, true)
             };
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/formats/renderer.rs" "/checkout/src/librustdoc/passes/strip_priv_imports.rs" "/checkout/src/librustdoc/formats/mod.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/passes/unindent_comments/tests.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs" "/checkout/src/librustdoc/clean/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
