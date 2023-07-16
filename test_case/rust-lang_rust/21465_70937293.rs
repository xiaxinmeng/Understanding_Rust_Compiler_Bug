
fn extend1<T, I: Iterator<Item=T>+ExactSizeIterator>(vec: &mut Vec<T>, iter: I) {
    let mut iter = iter;
    let len = iter.len();
    vec.reserve(len);
    let mut ptr = vec.as_mut_ptr().offset(vec.len());
    for el in iter {
       ptr.write(el);
       ptr = ptr.offset(1);
    }
    vec.set_len(len);
}
