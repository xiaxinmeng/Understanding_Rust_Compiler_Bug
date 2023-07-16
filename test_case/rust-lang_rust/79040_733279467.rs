
Some(mut err) => {
            // The parser provided a sub-optimal `HasPlaceholders` suggestion for the type.
            // We are typeck and have the real type, so remove that and suggest the actual type.
            match ty.kind() {
                ty::Error(_) => {
                    err.delay_as_bug();
                },
                _ => {
                    err.suggestions.clear();
                    err.span_suggestion(
                        span,
                        "provide a type for the item",
                        format!("{}: {}", item_ident, ty),
                        Applicability::MachineApplicable,
                    )
                    .emit();
                }
            }
        }
