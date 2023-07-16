rust
pub fn unfilled<'buf>(self: &'buf mut BorrowBuf<'data>) -> BorrowCursor<'buf, 'data>
