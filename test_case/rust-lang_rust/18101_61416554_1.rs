 rust
#[deriving(Clone)]
struct Map<A, B, I: Iterator<A>, F: FnMut(A) -> B> {
    iter: I,
    f: F,
}

impl<A, B, I: Iterator<A>, F: FnMut(A) -> B> Iterator<B> for Map<A, B, I, F> {
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(|elt| self.f.call_mut((elt,)))
    }
}
