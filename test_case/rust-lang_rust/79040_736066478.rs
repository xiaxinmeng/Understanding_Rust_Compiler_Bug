rust
        Some(mut err) => {
            err.suggestions.clear();
            err.span_suggestion(
                span,
                "provide a type for the item",
                format!("{}: {}", item_ident, ty),
                Applicability::MachineApplicable,
            )
            .emit_unless(matches!(ty.kind(), ty::Error(_)));
        }
