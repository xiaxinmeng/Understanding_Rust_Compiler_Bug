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
Diff in /checkout/compiler/rustc_typeck/src/check/method/mod.rs at line 546:
                         tcx.struct_span_lint_hir(FUTURE_PRELUDE_COLLISION, expr_id, span, |lint| {
                             let trait_def_id = pick.item.container.assert_trait();
                             let trait_generics = tcx.generics_of(trait_def_id);
-                            let parameter_count = trait_generics.count() - (trait_generics.has_self as usize);
+                            let parameter_count =
+                                trait_generics.count() - (trait_generics.has_self as usize);
                             let trait_name = if parameter_count == 0 {
                             let trait_name = if parameter_count == 0 {
                                 tcx.def_path_str(trait_def_id)
Diff in /checkout/compiler/rustc_typeck/src/check/method/mod.rs at line 554:
                                 format!(
                                     "{}<{}>",
                                     tcx.def_path_str(trait_def_id),
-                                    std::iter::repeat("_").take(parameter_count).collect::<Vec<_>>().join(", ")
+                                    std::iter::repeat("_")
+                                        .take(parameter_count)
+                                        .collect::<Vec<_>>()
+                                        .join(", ")
                             };
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/outlives/explicit.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_typeck/src/variance/solve.rs" "/checkout/compiler/rustc_typeck/src/check/method/probe.rs" "/checkout/compiler/rustc_typeck/src/variance/constraints.rs" "/checkout/compiler/rustc_typeck/src/check/method/mod.rs" "/checkout/compiler/rustc_typeck/src/variance/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
