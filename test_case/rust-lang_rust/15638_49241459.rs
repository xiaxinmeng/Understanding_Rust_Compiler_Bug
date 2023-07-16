 rust
#[deriving(Clone)]
pub struct CharItems<'a> {
    iter: slice::Items<'a, u8>
}

#[inline]
fn unwrap_or_0(opt: Option<&u8>) -> u8 {
    match opt {
        Some(&byte) => byte,
        None => 0,
    }
}

impl<'a> Iterator<char> for CharItems<'a> {
    #[inline]
    fn next(&mut self) -> Option<char> {
        fn decode_multibyte<'a>(x: u8, it: &mut slice::Items<'a, u8>) -> char {
            // NOTE: Performance is very sensitive to the exact formulation here
            // Decode from a byte combination out of: [[[x y] z] w]
            let CONT_MASK = 0x3F; // continuation byte mask
            let init = utf8_first_byte!(x, 2);
            let mut ch;
            let y = unwrap_or_0(it.next());
            ch = init << 6 | (y & CONT_MASK) as u32;
            if x >= 0xE0 {
                let z = unwrap_or_0(it.next());

                let y_z = (((y & CONT_MASK) as u32) << 6) | (z & CONT_MASK) as u32;
                ch = init << 12 | y_z;
                if x >= 0xF0 {
                    let w = unwrap_or_0(it.next());
                    ch = (init & 7) << 18 | y_z << 6 | (w & CONT_MASK) as u32;
                }
            }
            unsafe {
                mem::transmute(ch)
            }
        }

        match self.iter.next() {
            None => None,
            Some(&next_byte) => {
                if next_byte < 128 {
                    Some(next_byte as char)
                } else {
                    Some(decode_multibyte(next_byte, &mut self.iter))
                }
            }
        }
    }
}
