 rust
#[inline(never)]
fn extend7<T, I: Iterator<Item=T>+ExactSizeIterator>(vec: &mut Vec<T>, mut iter: I) {
    let mut iter = iter;
    let len = iter.len();
    vec.reserve(len);
    unsafe {
        let mut ptr = vec.as_mut_ptr().offset(vec.len() as isize);
        for el in iter {
           ptr::write(ptr, el);
           ptr = ptr.offset(1);
        }
        vec.set_len(len);
    }
}
