 rust
pub struct Citems<'a> {
    iter: std::slice::Items<'a, u8>
}

impl<'a> Iterator<char> for Citems<'a>
{
    #[inline]
    fn next(&mut self) -> Option<char>
    {
        #[inline(never)]
        fn decode_multibyte<'a>(fst: u8, it: &mut std::slice::Items<'a, u8>)
            -> Option<char>
        {
            let w = UTF8_CHAR_WIDTH[fst as uint];
            let mut ch = utf8_first_byte!(fst, w as uint);
            ch = utf8_acc_cont_byte!(ch, it.next().map_or(0, |x| *x));
            if w > 2 {
                ch = utf8_acc_cont_byte!(ch, it.next().map_or(0, |x| *x));
                if w > 3 {
                    ch = utf8_acc_cont_byte!(ch, it.next().map_or(0, |x| *x));
                }
            }
            unsafe {
                Some(transmute(ch))
            }
        }

        match self.iter.next() {
            Some(&next_byte) => {
                if next_byte < 128 {
                    Some(next_byte as char)
                } else {
                    decode_multibyte(next_byte, &mut self.iter)
                }
            }
            None => None
        }
    }
}
