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
Diff in /checkout/compiler/rustc_typeck/src/check/method/suggest.rs at line 932:
                                     spans.into()
                                 };
                                 if let Some(trait_ref) = of_trait {
-                                    spans.push_span_label(
-                                        trait_ref.path.span,
-                                    );
-                                    );
+                                    spans.push_span_label(trait_ref.path.span, String::new());
-                                spans.push_span_label(
-                                    self_ty.span,
-                                    String::new(),
-                                );
-                                );
+                                spans.push_span_label(self_ty.span, String::new());
 
                                 let entry = spanned_predicates.entry(spans.into());
                                 entry.or_insert_with(|| (path, tr_self_ty, Vec::new())).2.push(p);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/mod.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsic.rs" "/checkout/compiler/rustc_typeck/src/coherence/unsafety.rs" "/checkout/compiler/rustc_typeck/src/check/coercion.rs" "/checkout/compiler/rustc_typeck/src/coherence/mod.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs" "/checkout/compiler/rustc_mir_transform/src/add_moves_for_packed_drops.rs" "/checkout/compiler/rustc_typeck/src/check/method/suggest.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
