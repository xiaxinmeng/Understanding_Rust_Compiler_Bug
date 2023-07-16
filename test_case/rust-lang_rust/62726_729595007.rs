rust
pub fn advance<'b>(
    bufs: &'b mut [IoSlice<'a>],
    n: usize
) -> &'b mut [IoSlice<'a>]
