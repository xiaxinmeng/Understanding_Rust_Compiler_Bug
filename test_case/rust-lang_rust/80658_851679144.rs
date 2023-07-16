rust
            fn add_labels_for_types(
                &self,
                err: &mut DiagnosticBuilder<'_>,
                target: &str,
                types: &FxHashMap<TyCategory, FxHashSet<Span>>,
            ) {
                for (key, values) in types.iter() {
                    let count = values.len();
                    let kind = key.descr();
                    let mut returned_async_output_error = false;
                    for sp in values {
                        err.span_label(
                            *sp,
                            format!(
                                "{}{}{} {}{}",
                                if sp.is_desugaring(DesugaringKind::Async)
                                    && !returned_async_output_error
                                {
                                    "checked the `Output` of this `async fn`, "
                                } else if count == 1 {
                                    "the "
                                } else {
                                    ""
                                },
                                if count > 1 { "one of the " } else { "" },
                                target,
                                kind,
                                pluralize!(count),
                            ),
                        );
                        if sp.is_desugaring(DesugaringKind::Async)
                            && returned_async_output_error == false
                        {
                            err.note("while checking the return type of the `async fn`");
                            returned_async_output_error = true;
                        }
                    }
                }
            }
        }
