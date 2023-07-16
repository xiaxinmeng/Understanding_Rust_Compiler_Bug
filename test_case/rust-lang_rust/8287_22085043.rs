
// can't use bytes!() here
static HEXDIGITS: [u8, ..16] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102];

fn to_hex_C(&self) -> ~str {
    let mut v = vec::with_capacity(self.len() * 2);
    for &byte in self.iter() {
        v.push(HEXDIGITS[byte >> 4]);
        v.push(HEXDIGITS[byte & 0xf]);
    }
    unsafe {
        str::raw::from_bytes_owned(v)
    }
}
