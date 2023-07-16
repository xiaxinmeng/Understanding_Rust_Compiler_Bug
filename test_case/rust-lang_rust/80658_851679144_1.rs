rust
            (_, Some(ty)) if ty::TyS::same_type(exp_found.expected, ty) => {
                let span = match cause.code {
                    // scrutinee's span
                    ObligationCauseCode::Pattern { span: Some(span), .. } => span,
                    _ => exp_span,
                };
                diag.span_suggestion_verbose(
                    span.shrink_to_hi(),
                    "consider `await`ing on the `Future`",
                    ".await".to_string(),
                    Applicability::MaybeIncorrect,
                );
            }
