rust
pub fn find<'a, P>(&'a self, pat: P) -> Option<usize> where
    P: Pattern<'a>;
