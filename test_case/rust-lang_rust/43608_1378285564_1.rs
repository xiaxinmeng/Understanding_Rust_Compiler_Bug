rust
let mut multi_span: MultiSpan = parent_expr.span.into();
multi_span.push_span_label(args[arg_idx].span, format!("this argument influences the return type of `{}`", path.segments[0].ident));
err.span_help(multi_span, ...);
