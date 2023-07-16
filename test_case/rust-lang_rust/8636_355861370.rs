rust
#![feature(slice_patterns)]
fn mut_head_tail<'a, A>(v: &'a mut [A]) -> Option<(&'a mut A, &'a mut [A])> {
    match *v {
        [ref mut head, ref mut tail..] => {
            Some((head, tail))
        }
        [..] => None
    }
}
