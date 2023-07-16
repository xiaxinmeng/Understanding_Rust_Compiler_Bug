 rust
impl From<Vec<char>> for String {
    fn from(mut v: Vec<char>) -> String {
        unsafe {
            let ptr = v.as_mut_ptr() as *mut u8;
            let mut bytes = 0;
            {
            let mut rest = v.as_mut_slice();
            while let Some((chr, rest_)) = {rest}.split_first_mut() {
                for byte in chr.encode_utf8() {
                    *ptr.offset(bytes) = byte;
                    bytes += 1;
                }
                rest = rest_;
            }
            }
            let cap = v.capacity();
            ::std::mem::forget(v);
            String::from_raw_parts(ptr, bytes as usize, cap)
        }
    }
} 
// Perhaps this code could be made better, I didn’t ponder much on it.
