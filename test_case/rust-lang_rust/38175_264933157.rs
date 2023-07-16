rust
fn extend(v: &mut Vec<u8>, input: &[u8]) {
    v.reserve(input.len());
    unsafe {
        let dst = v.as_mut_ptr();
        let mut i = v.len() as isize;
        let mut iter = input.iter().map(|&x| x);
        while let Some(elt) = iter.next() {
            *dst.offset(i) = elt;
            i += 1;
        }
        let new_len = v.len() + input.len();
        v.set_len(new_len);
    }
}
