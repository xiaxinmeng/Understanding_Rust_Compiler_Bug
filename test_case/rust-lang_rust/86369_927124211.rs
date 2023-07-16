rust
#![feature(cursor_remaining)]

use std::io::{self, BufRead, Cursor, Read};

struct Asset<'file> {
    data: &'file [u8],
}

impl<'file> Asset<'file> {
    fn load(cursor: &mut Cursor<&'file [u8]>) -> io::Result<Self> {
        let mut len = [0u8; 2];
        cursor.read_exact(&mut len[..])?;
        let len = u16::from_le_bytes(len) as usize;

        let data = cursor.remaining_slice().get(..len)
            .ok_or(io::ErrorKind::UnexpectedEof)?;
        cursor.consume(len);

        Ok(Self { data })
    }
}
