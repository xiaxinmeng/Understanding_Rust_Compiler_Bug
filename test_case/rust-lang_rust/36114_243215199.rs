 rust
struct_span_err!(tcx.sess, span, E0393,
                 "the type parameter `{}` must be explicitly specified",
                 def.name)
    .span_label(span, &format!("missing reference to `{}`", def.name))
    .emit();
