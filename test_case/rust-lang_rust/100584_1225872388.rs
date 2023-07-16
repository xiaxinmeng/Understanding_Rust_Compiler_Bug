rust
err.multipart_suggestion(
    vec![
        (lit_expr.span.shrink_to_lo(), "format!(".to_string()),
        (lit_expr.span.shrink_to_hi(), ")".to_string()),
    ],
    "if you meant to use string interpolation in the string literal, use the `format!` marco",
    Applicability::MachineApplicable,
);
