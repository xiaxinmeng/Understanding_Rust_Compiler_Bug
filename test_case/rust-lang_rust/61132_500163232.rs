rust
#[derive(AsError)]
#[error = "Cannot move {ty} out of borrow", span]
#[note = "First borrwed here", other_span]
#[code = E12345]
struct MoveOutOfBorrowError {
    ty: Ty,
    span: Span,
    other_span: Span,
}
