
Some(mut err) => {
            err.suggestions.clear();
            err.span_suggestion(
                span,
                "provide a type for the item",
                format!("{}: {}", item_ident, ty),
                Applicability::MachineApplicable,
            )
            .emit_unless(
                match ty.kind() {
                    ty::Error(_) => {true},
                    _ => {false}  // Is _ ok here?
                }
            );
        }
