rust
#[inline(never)]
pub fn sum_byteorder_cursor_impl(cursor: &mut Cursor<&[u8]>) -> io::Result<u64> {
    let mut ret = 0;
    ret += cursor.read_u8()? as u64;
    ret += cursor.read_u8()? as u64;
    ret += cursor.read_u8()? as u64;
    ret += cursor.read_u8()? as u64;
    // ret += cursor.read_u16::<LittleEndian>()? as u64;
    // ret += cursor.read_u32::<LittleEndian>()? as u64;
    Ok(ret)
}
