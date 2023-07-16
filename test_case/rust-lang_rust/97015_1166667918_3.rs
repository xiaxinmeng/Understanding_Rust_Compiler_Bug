rust
fn split_at_unfilled<'buf>(
    self: &'buf mut BorrowBuf<'data>,
) -> (&'buf [u8], BorrowCursor<'buf, 'data>)
