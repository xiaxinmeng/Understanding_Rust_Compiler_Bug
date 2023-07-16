rust
fn get_full_buffer<'data>(mut buf: BorrowBuf<'data>) -> &'data mut [u8] {
    buf.clear();
    let mut cursor = buf.unfilled_mut(); // `self`-based / consumes `buf`
    cursor.ensure_init();
    cursor.init_mut() //  `self`-based
}
