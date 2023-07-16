 rust
pub trait StreamingIterator<T<'*>> {
    fn next<'a>(&'a mut self) -> Option<T<'a>>;

    // Can we implement _any_ reasonable version of `fold` here? 
    fn fold<'a,S>(&'a mut self, init: B, f: |S, T<'a>| -> S) -> S { ... } 
}
