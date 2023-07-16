 rust
#[inline(never)]
fn extend1<T, I: Iterator<Item=T>+ExactSizeIterator>(vec: &mut Vec<T>, iter: I) {
    let mut iter = iter;
    let len = iter.len();
    vec.reserve(len);
    for _ in (0..len) {
        let el = iter.next();
        unsafe {
            assume(el.is_some());
            assume(vec.len() < vec.capacity());
            vec.push(el.unwrap())
        }
    }
}
