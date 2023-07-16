 rust
fn filter<P>(self, predicate: P) -> Filter<A, Self, P> where P: FnMut(&A)
