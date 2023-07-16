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
Diff in /checkout/compiler/rustc_lint/src/mem_discriminant_non_enum.rs at line 1:
-use crate::{LateContext, LateLintPass, context::LintContext};
+use crate::{context::LintContext, LateContext, LateLintPass};
 use rustc_hir as hir;
 use rustc_span::symbol::sym;
Diff in /checkout/compiler/rustc_lint/src/mem_discriminant_non_enum.rs at line 39:
Diff in /checkout/compiler/rustc_lint/src/mem_discriminant_non_enum.rs at line 39:
                     if cx.tcx.is_diagnostic_item(sym::mem_discriminant, def_id) {
                         let ty_param = cx.typeck_results().node_substs(func.hir_id).type_at(0);
                         if !ty_param.is_enum() {
-                            cx.struct_span_lint(
-                                MEM_DISCRIMINANT_NON_ENUM,
-                                expr.span,
-                                |builder| {
-                                    builder.build(
+                            cx.struct_span_lint(MEM_DISCRIMINANT_NON_ENUM, expr.span, |builder| {
+                                    .build(
+                                    .build(
                                         "the result of calling `mem::descriminant` with a \
-                                            non-enum type is unspecified"
+                                            non-enum type is unspecified",
                                     )
                                     .span_note(
                                         args[0].span,
Diff in /checkout/compiler/rustc_lint/src/mem_discriminant_non_enum.rs at line 57:
                                     )
                                     )
                                     .emit();
-                            );
+                            });
                         }
                     }
                     }
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/early.rs" "/checkout/compiler/rustc_ty_utils/src/needs_drop.rs" "/checkout/compiler/rustc_lint/src/redundant_semicolon.rs" "/checkout/compiler/rustc_ty_utils/src/ty.rs" "/checkout/compiler/rustc_lint/src/mem_discriminant_non_enum.rs" "/checkout/compiler/rustc_lint/src/context.rs" "/checkout/compiler/rustc_ty_utils/src/instance.rs" "/checkout/compiler/rustc_ty_utils/src/common_traits.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
