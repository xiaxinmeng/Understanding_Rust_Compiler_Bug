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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 450:
                 {
                     // Missing generic type parameter bound.
                     let param_name = self_ty.to_string();
-                    let constraint = with_no_trimmed_paths!(trait_pred
-                        .print_modifiers_and_trait_path()
-                        .to_string());
+                    let constraint = with_no_trimmed_paths!(
+                        trait_pred.print_modifiers_and_trait_path().to_string()
                     if suggest_constraining_type_param(
                         self.tcx,
                         generics,
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 1062:
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 1062:
             _ => return None,
         };
 
-        if let hir::FnRetTy::Return(ret_ty) = sig.decl.output {
-            Some(ret_ty.span)
-            None
-        }
-        }
+        if let hir::FnRetTy::Return(ret_ty) = sig.decl.output { Some(ret_ty.span) } else { None }
 
 
     /// If all conditions are met to identify a returned `dyn Trait`, suggest using `impl Trait` if
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/prove_predicate.rs" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
