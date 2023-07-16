rust

#[inline]
fn write(buf: &mut [u8], data: &[u8])  {
    // With this condition it optimizes to a store, without it we get memcpy calls.
    if buf.len() < 1 {
        return;
    }    
    // This also gets rid of the memcpy
    // let amt = data.len();
    // But this doesn't.
    // let amt = buf.len();
    let amt = cmp::min(data.len(), buf.len());
    buf.copy_from_slice(&data[..amt]);
}

pub fn write_byte(buf: &mut [u8], byte: u8) {
    write(buf, &[byte]);
}

