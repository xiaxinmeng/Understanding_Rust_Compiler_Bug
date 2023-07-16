rust
let mut diag = cx.ecx.struct_span_err(
    errs.iter().map(|&(sp, _)| sp).collect::<Vec<Span>>(),
    "multiple unused formatting arguments"
);
