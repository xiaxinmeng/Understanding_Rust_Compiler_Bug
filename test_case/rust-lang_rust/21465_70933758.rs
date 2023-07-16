 rust
#[inline(never)]
fn extend4<T, I: RawIterator<Item=T>>(vec: &mut Vec<T>, iter: I) {
    let mut iter = iter; // Add this line
    let len = iter.len();
    vec.reserve(len);
    unsafe {
        let mut ptr = vec.as_mut_ptr().offset(vec.len() as isize);
        let end = ptr.offset(len as isize);
        while ptr != end {
            ptr::write(ptr, iter.next());
            ptr = ptr.offset(1);
        }
    }
}
