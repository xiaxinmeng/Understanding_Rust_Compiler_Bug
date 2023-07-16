rust
fn extend(v: &mut Vec<u8>, input: &[u8]) {
    v.reserve(input.len());
    unsafe {
        let mut dst = v.as_mut_ptr().offset(v.len() as isize);
        let mut iter = input.iter().map(|&x| x);
        while let Some(elt) = iter.next() {
            *dst = elt;
            dst = dst.offset(1);
        }
        let new_len = v.len() + input.len();
        v.set_len(new_len);
    }
}

#[no_mangle]
pub fn extend_1(v: &mut Vec<u8>, data: &[u8; 16]) {
    extend(v, data);
}
