 rust
#[inline(never)]
fn extend2<T, I: Iterator<Item=T>+ExactSizeIterator>(vec: &mut Vec<T>, iter: I) {
    let mut iter = iter;
    let len = iter.len();
    vec.reserve(len);
    unsafe {
        let mut ptr = vec.as_mut_ptr().offset(vec.len() as isize);
        let end = ptr.offset(len as isize);
        while ptr != end {
            let el = iter.next();
            assume(el.is_some());
            ptr::write(ptr, el.unwrap());
            ptr = ptr.offset(1);
        }
    }
}
