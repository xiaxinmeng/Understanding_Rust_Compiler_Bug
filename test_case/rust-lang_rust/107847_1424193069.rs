rust
const LEN: usize = 64;

pub fn array_index_unchecked(v: [u8; LEN]) -> bool {
    for i in 0..LEN {
        if unsafe { *v.get_unchecked(i) } != 0 {
            return false;
        }
    }

    true
}
