 rust
pub struct Block {
    view_items: ~[view_item],
    stmts: ~[@Stmt],
    expr: Option<@Expr>,
    id: NodeId,
    rules: BlockCheckMode,
    span: Span,
}
