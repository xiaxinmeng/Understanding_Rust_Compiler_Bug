 rust
struct LossyUTF8Iter<'a> { bytes: &'a [u8] }
fn lossy_utf8(&'a [u8]) -> LossyUTF8Iter<'a> { /* obvious */ }

impl<'a> Iterator<&'a str> for LossyUTF8Iter<'a> {
    fn next(&mut self) -> Option<&'a str> {
        static REP: &'static str = "\uFFFD\uFFFD\uFFFD\uFFFD"; // some reasonable length

        let good = /* validate first character */;

        if good {
            /* find first invalid byte... */

            return Some(self.bytes.slice_to(index));
        } else {
            /* find first valid character (or 
               REP.char_len() invalid characters,
               whichever comes first)... */

            return Some(REP.slice_to(3 * replacement_count));
        }
    }
}

// helper, not necessary (and won't work yet)
fn slice_if_possible<'a, It: Iterator<&'a str>>(it: It) -> MaybeOwned<'a> {
     // if it has 1 element, return that; otherwise concatentate them all in a new alloc.
}
