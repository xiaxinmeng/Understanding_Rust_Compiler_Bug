rust
impl Write for Cursor<&mut [u8]> {}
impl Write for Cursor<&mut Vec<u8>> {}
impl Write for Cursor<Vec<u8>> {}
impl Write for Cursor<Box<[u8]>> {}
