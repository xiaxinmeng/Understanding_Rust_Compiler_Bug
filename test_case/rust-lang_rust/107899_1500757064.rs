
if let Some(sp) = outer_span {
err.span_label(sp, "if and else have incompatible types");
}
