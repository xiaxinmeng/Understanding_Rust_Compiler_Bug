rust
fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>
where
    P: FnMut(Self::Item) -> Option<B>
