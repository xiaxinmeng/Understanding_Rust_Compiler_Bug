 rust
let mut err = struct_span_err!(tcx.sess, span, E0393,
                               "the type parameter `{}` must be explicitly specified",
                               def.name);
err.span_label(span, &format!("missing reference to `{}`", def.name));
err.note(&format!("because of the default `Self` reference, type parameters must be specified on object types"));
err.emit();
