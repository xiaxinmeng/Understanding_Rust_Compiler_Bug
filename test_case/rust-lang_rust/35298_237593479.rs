 rust
let mut err = struct_span_err!(tcx.sess, item.span, E0120,
                                                  "the Drop trait may only be implemented on structures");
match item.node {
    ItemImpl(_, _, _, _, ref ty, _) => {
        err.span = ty.span;
        err.span_label(ty.span, &format!("implementing Drop requires a struct"));
    },
    _ => {
        err.span_label(item.span, &format!("implementing Drop requires a struct"));
    }
};
err.emit();
