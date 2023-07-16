
match ty.kind() {
        TyKind::Err | TyKind::FnDef | TyKind::Closure | ... => { }
        _ =>    {
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
