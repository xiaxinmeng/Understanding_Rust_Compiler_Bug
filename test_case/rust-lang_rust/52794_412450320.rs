rust
fn dispatch<Op: Operation, Response>(
    &self,
    op: &Operation<Response = Response>,
) -> Response
