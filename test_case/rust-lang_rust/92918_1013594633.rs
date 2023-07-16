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
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 489:
                             debug!(?param, "unelided lifetime in signature");
 
                             // This indicates an illegal lifetime in a non-assoc-trait position
-                            tcx.sess
-                                .delay_span_bug(self.span, "unelided lifetime in signature");
+                            tcx.sess.delay_span_bug(self.span, "unelided lifetime in signature");
 
                             // Supply some dummy value. We don't have an
                             // `re_error`, annoyingly, so use `'static`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/coherence/builtin.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs" "/checkout/compiler/rustc_typeck/src/coherence/unsafety.rs" "/checkout/compiler/rustc_typeck/src/coherence/mod.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_typeck/src/check/coercion.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
