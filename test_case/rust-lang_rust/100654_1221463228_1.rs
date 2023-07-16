
+                    if !matches!(
+                        span.ctxt().outer_expn_data().kind,
+                        ExpnKind::Root | ExpnKind::Desugaring(DesugaringKind::ForLoop)
+                    ) {
+                    }
+                    }
+                    if snippet.starts_with('&') {
+                        // This is already a literal borrow and the obligation is failing
+                        // somewhere else in the obligation chain. Do not suggest non-sense.
+                    }
+                    }
+                    // We have a very specific type of error, where just borrowing this argument
+                    // might solve the problem. In cases like this, the important part is the
+                    // original type obligation, not the last one that failed, which is arbitrary.
+                    // Because of this, we modify the error to refer to the original obligation and
+                    // return early in the caller.
-                        let msg = format!("the trait bound `{}` is not satisfied", old_pred);
-                        if has_custom_message {
-                        if has_custom_message {
-                            err.note(&msg);
-                            err.message =
-                            err.message =
-                                vec![(rustc_errors::DiagnosticMessage::Str(msg), Style::NoStyle)];
-                        err.span_label(
-                            span,
-                            format!(
-                            format!(
-                                "the trait `{}` is not implemented for `{}`",
-                                old_pred.print_modifiers_and_trait_path(),
-                                old_pred.self_ty().skip_binder(),
+                    let msg = format!("the trait bound `{}` is not satisfied", old_pred);
+                    if has_custom_message {
+                        err.note(&msg);
+                        err.message =
+                        err.message =
+                            vec![(rustc_errors::DiagnosticMessage::Str(msg), Style::NoStyle)];
+                    err.span_label(
+                        span,
+                        format!(
+                        format!(
+                            "the trait `{}` is not implemented for `{}`",
+                            old_pred.print_modifiers_and_trait_path(),
+                            old_pred.self_ty().skip_binder(),
+                    );
+
+
+                    if imm_ref_self_ty_satisfies_pred && mut_ref_self_ty_satisfies_pred {
+                        err.span_suggestions(
+                            span.shrink_to_lo(),
+                            "consider borrowing here",
+                            ["&".to_string(), "&mut ".to_string()].into_iter(),
+                        );
+                    } else {
+                    } else {
+                        let is_mut = mut_ref_self_ty_satisfies_pred || ref_inner_ty_mut;
+                        err.span_suggestion_verbose(
+                            span.shrink_to_lo(),
+                            &format!(
+                                "consider{} borrowing here",
+                                if is_mut { " mutably" } else { "" }
                             ),
+                            format!("&{}", if is_mut { "mut " } else { "" }),
                         );
-
-
-                        if imm_ref_self_ty_satisfies_pred && mut_ref_self_ty_satisfies_pred {
-                            err.span_suggestions(
-                                span.shrink_to_lo(),
-                                "consider borrowing here",
-                                ["&".to_string(), "&mut ".to_string()].into_iter(),
-                            );
-                        } else {
-                        } else {
-                            let is_mut = mut_ref_self_ty_satisfies_pred || ref_inner_ty_mut;
-                            err.span_suggestion_verbose(
-                                span.shrink_to_lo(),
-                                &format!(
-                                    "consider{} borrowing here",
-                                    if is_mut { " mutably" } else { "" }
-                                ),
-                                format!("&{}", if is_mut { "mut " } else { "" }),
-                            );
-                        }
-                        return true;
                     }
---
+            }
+            return false;
+        };
 
         if let ObligationCauseCode::ImplDerivedObligation(cause) = &*code {
             try_borrowing(cause.derived.parent_trait_pred, &[])
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
