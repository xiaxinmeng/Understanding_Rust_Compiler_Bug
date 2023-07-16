rust
let mut err = cx.struct_span_lint(NON_UPPER_CASE_GLOBALS, span, &format!("{} `{}` should have an upper case name");
err.span_suggestion(span, "use an uppercase name", uc);
err.emit();
