
match tcx.sess.diagnostic().steal_diagnostic(span, StashKey::ItemNoType) {
        Some(mut err) => {
            // The parser provided a sub-optimal `HasPlaceholders` suggestion for the type.
            // We are typeck and have the real type, so remove that and suggest the actual type.
            err.suggestions.clear();
            err.span_suggestion(
                span,
                "provide a type for the item",
                format!("{}: {}", item_ident, ty),
                Applicability::MachineApplicable,
            )
            .emit();
        }
        None => {
            let mut diag = bad_placeholder_type(tcx, vec![span]);
            if !matches!(ty.kind(), ty::Error(_)) {
                diag.span_suggestion(
                    span,
                    "replace `_` with the correct type",
                    ty.to_string(),
                    Applicability::MaybeIncorrect,
                );
            }
            diag.emit();
        }
    }
