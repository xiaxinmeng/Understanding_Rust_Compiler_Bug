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
Diff in /checkout/compiler/rustc_typeck/src/expr_use_visitor.rs at line 271:
                                         // If the pat kind is a Path we want to check whether the
                                         // variant contains at least one field. If that's the case,
                                         // we want to borrow discr.
-                                        if matches!(pat.kind, PatKind::Path(..)) && variant.fields.len() > 0 {
+                                        if matches!(pat.kind, PatKind::Path(..))
+                                            && variant.fields.len() > 0
+                                        {
                                             needs_to_be_read = true;
                                     }
                                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/coherence/mod.rs" "/checkout/compiler/rustc_typeck/src/coherence/unsafety.rs" "/checkout/compiler/rustc_typeck/src/coherence/orphan.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_typeck/src/expr_use_visitor.rs" "/checkout/compiler/rustc_typeck/src/coherence/builtin.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
