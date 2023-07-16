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
Diff in /checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs at line 153:
         if !self.tcx.hir().rustc_coherence_is_core() {
             if self.tcx.features().rustc_attrs {
                 for item in items {
-                    if !self.tcx.has_attr(item.id.def_id.to_def_id(), sym::rustc_allow_incoherent_impl) {
+                    if !self
+                        .tcx
+                        .has_attr(item.id.def_id.to_def_id(), sym::rustc_allow_incoherent_impl)
                         struct_span_err!(
                             self.tcx.sess,
                             span,
                             span,
Diff in /checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs at line 160:
                             E0390,
                             "cannot define inherent `impl` for primitive types outside of `core`",
-                        ).help(INTO_CORE).span_help(item.span, ADD_ATTR).emit();
+                        )
+                        .help(INTO_CORE)
+                        .span_help(item.span, ADD_ATTR)
+                        .emit();
                     }
                 }
                 }
Diff in /checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs at line 169:
                     E0390,
                     E0390,
                     "cannot define inherent `impl` for primitive types",
-                ).help("consider using an extension trait instead").emit();
+                )
+                .help("consider using an extension trait instead")
+                .emit();
         }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_typeck/src/coherence/orphan.rs" "/checkout/compiler/rustc_typeck/src/check_unused.rs" "/checkout/compiler/rustc_typeck/src/expr_use_visitor.rs" "/checkout/compiler/rustc_typeck/src/variance/solve.rs" "/checkout/compiler/rustc_typeck/src/coherence/builtin.rs" "/checkout/compiler/rustc_typeck/src/coherence/mod.rs" "/checkout/compiler/rustc_traits/src/chalk/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
