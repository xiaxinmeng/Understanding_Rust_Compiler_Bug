
#[derive(AsError)]
#[code = E12345]
struct MoveOutOfBorrowError {
    ty: Ty,
    #[error = "cannot move {ty} out of borrow"]
    #[label = "cannot move out of borrow"]
    span: Span,
    #[label = "`{ty}` first borrowed here"]
    other_span: Span,
    #[suggestion(msg = "consider cloning here", code = "{opt_sugg}.clone()")]
    opt_sugg: Option<Span>,
}
