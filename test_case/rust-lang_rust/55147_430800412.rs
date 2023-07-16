rust
pub fn unchecked(dst: &mut [u8], offset: usize) {
    let mut i = offset;
    if i.checked_add(4).unwrap() > dst.len() {
        unsafe {
            *(dst.get_unchecked_mut(i)) = 1;
            i += 1;
            *(dst.get_unchecked_mut(i)) = 2;
            i += 1;
            *(dst.get_unchecked_mut(i)) = 3;
            i += 1;
            *(dst.get_unchecked_mut(i)) = 4;
        }
    }
}

pub fn checked(dst: &mut [u8], offset: usize) {
    let mut i = offset;
    if i.checked_add(4).unwrap() > dst.len() {
        dst[i] = 1;
        i += 1;
        dst[i] = 2;
        i += 1;
        dst[i] = 3;
        i += 1;
        dst[i] = 4;
    } 
}
