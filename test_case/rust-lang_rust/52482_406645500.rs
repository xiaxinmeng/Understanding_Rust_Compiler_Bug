rust
let source_ty = cx.tables().expr_ty(source);
if source_ty == expr_ty {
    // Convert the lexpr to a vexpr.
    ExprKind::Use { source: source.to_ref() }
else {
...
