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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 18:
 use rustc_hir::{AsyncGeneratorKind, GeneratorKind, Node};
 use rustc_middle::ty::{
     self, suggest_arbitrary_trait_bound, suggest_constraining_type_param, AdtKind, DefIdTree,
-    Infer, InferTy, ToPredicate, Ty, TyCtxt, TypeFoldable, WithConstness
+    Infer, InferTy, ToPredicate, Ty, TyCtxt, TypeFoldable, WithConstness,
 };
 use rustc_middle::ty::{TypeAndMut, TypeckResults};
 use rustc_span::def_id::LOCAL_CRATE;
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 998:
         };
 
 
-        if let hir::FnRetTy::Return(ret_ty) = sig.decl.output {
-            Some(ret_ty.span)
-            None
-        }
-        }
+        if let hir::FnRetTy::Return(ret_ty) = sig.decl.output { Some(ret_ty.span) } else { None }
 
 
     /// If all conditions are met to identify a returned `dyn Trait`, suggest using `impl Trait` if
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 1863:
             GeneratorInteriorOrUpvar::Upvar(upvar_span) => {
                 // `Some(ref_ty)` if `target_ty` is `&T` and `T` fails to impl `Sync`
                 let refers_to_non_sync = match target_ty.kind() {
-                    ty::Ref(_, ref_ty, _) => {
-                        match self.evaluate_obligation(&obligation) {
-                            Ok(eval) => {
-                                if eval.may_apply() {
-                                } else {
-                                    Some(ref_ty)
-                                }
-                            },
-                            },
-                            _ => None
+                    ty::Ref(_, ref_ty, _) => match self.evaluate_obligation(&obligation) {
+                        Ok(eval) => {
+                            if eval.may_apply() {
+                            } else {
+                                Some(ref_ty)
+                            }
                         }
                         }
+                        _ => None,
                     },
                     _ => None,
                 };
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 1882:
                     Some(ref_ty) => {
                         span.push_span_label(
                             upvar_span,
-                            format!("has type `{}` -- refers to `{}` which is not `Sync`", target_ty, ref_ty),
+                            format!(
+                                "has type `{}` -- refers to `{}` which is not `Sync`",
+                                target_ty, ref_ty
                         );
                         );
                         err.span_note(span, &format!("captured value {} because `&` references cannot be sent unless their referent is `Sync`", trait_explanation));
+                    }
                     None => {
                     None => {
                         span.push_span_label(
                             upvar_span,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/ascribe_user_type.rs" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/custom.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
